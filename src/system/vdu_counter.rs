use std::sync::Arc;
use parking_lot::Mutex;
use crate::memory::io_map::VideoIO;
use crate::renderer::LCDCMask;
use crate::system::clock_event::ClockEvent;

/*
 * This will have to be changed quite a bit for GBC double speed support
 */


pub enum VDUCounter {
    LCDOn { video_io: Arc<Mutex<VideoIO>>, line_counter: u32 },
    LCDOff { video_io: Arc<Mutex<VideoIO>>, generic_frame_counter: u32 },
}

impl VDUCounter {
    pub fn new(video_io: Arc<Mutex<VideoIO>>) -> Self {
        let lcd_enabled = LCDCMask::mask(video_io.lock().get_lcd_ctrl(), LCDCMask::LCD_ENABLE);

        if lcd_enabled {
            VDUCounter::LCDOn { video_io, line_counter: 0 }
        }
        else {
            VDUCounter::LCDOff { video_io, generic_frame_counter: 0 }
        }
    }

    pub fn tick (&mut self) -> Vec<ClockEvent>{ //I really hate the look of this function but the borrow checker shouts if some of it is split up
        let mut clock_events = Vec::new();

        match self {
            VDUCounter::LCDOn { video_io, line_counter } => {
                if LCDCMask::mask(video_io.lock().get_lcd_ctrl(), LCDCMask::LCD_ENABLE) {
                    if *line_counter % 2 == 0 {
                        clock_events.push(ClockEvent::CPUClock)
                    }

                    if *line_counter == 0 {
                        Self::reset_line_counter(video_io, &mut clock_events, line_counter);
                    } else {
                        *line_counter -= 1;
                    }
                }
                else {
                    if *line_counter % 2 == 0 {
                        clock_events.push(ClockEvent::CPUClock);
                        clock_events.push(ClockEvent::SendFrame);

                        video_io.lock().set_ly(0);

                        *self = VDUCounter::new(video_io.clone());
                    }
                    else {
                        *line_counter -= 1;
                    }
                }
            }
            VDUCounter::LCDOff { video_io, generic_frame_counter } => {
                if !LCDCMask::mask(video_io.lock().get_lcd_ctrl(), LCDCMask::LCD_ENABLE) {
                    if *generic_frame_counter % 2 == 0 {
                        clock_events.push(ClockEvent::CPUClock)
                    }

                    if *generic_frame_counter == 0 {
                        *generic_frame_counter = 34996;
                        clock_events.push(ClockEvent::SendFrame)
                    }
                    else {
                        *generic_frame_counter -= 1;
                    }
                }
                else {
                    if *generic_frame_counter % 2 == 0 {
                        clock_events.push(ClockEvent::CPUClock);
                        clock_events.push(ClockEvent::DrawLine);
                        *self = VDUCounter::new(video_io.clone());
                    }
                    else {
                        *generic_frame_counter -= 1;
                    }
                }
            }
        }

        clock_events
    }

    fn reset_line_counter (video_io: &mut Arc<Mutex<VideoIO>>, clock_events: &mut Vec<ClockEvent>, counter: &mut u32) {
        let old_ly = video_io.lock().get_ly();
        let ly = old_ly + 1;
        video_io.lock().set_ly(ly);


        if ly < 0x90 {
            clock_events.push(ClockEvent::DrawLine);
            *counter = 224;
        } else if ly == 0x90 {
            clock_events.push(ClockEvent::VBlankInterrupt);
            clock_events.push(ClockEvent::SendFrame);
            *counter = 274;
        } else if ly <= 0x98 {
            *counter = 274;
        } else {
            video_io.lock().set_ly(0);
            clock_events.push(ClockEvent::DrawLine);

            *counter = 274 + 224;
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::memory::MemoryTrait;
    use super::*;

    #[test]
    fn creates_lcdon_when_lcdc_on () {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);

        let vdu_counter = VDUCounter::new(video_io.clone());

        assert!(matches!(vdu_counter, VDUCounter::LCDOn { .. }));
    }

    #[test]
    fn creates_lcdoff_when_lcdc_off () {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x00);

        let vdu_counter = VDUCounter::new(video_io.clone());

        assert!(matches!(vdu_counter, VDUCounter::LCDOff { .. }));
    }

    #[test]
    fn lcdoff_turns_to_lcdon_when_lcdc_changes_on_even_tick () {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        video_io.lock().set(0xFF40, 0x80);
        let mut vdu_counter =VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 2 };

        video_io.lock().set(0xFF40, 0x00);
        vdu_counter.tick();

        assert!(matches!(vdu_counter, VDUCounter::LCDOff { .. }));
    }

    #[test]
    fn lcdoff_does_not_turn_to_lcdon_when_lcdc_changes_on_odd_tick () {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        video_io.lock().set(0xFF40, 0x80);
        let mut vdu_counter =VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 1 };

        video_io.lock().set(0xFF40, 0x00);
        vdu_counter.tick();

        assert!(matches!(vdu_counter, VDUCounter::LCDOn { .. }));
    }

    #[test]
    fn lcdon_turns_to_lcdoff_when_lcdc_changes_on_even_tick () {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        video_io.lock().set(0xFF40, 0x00);
        let mut vdu_counter =VDUCounter::LCDOff { video_io: video_io.clone(), generic_frame_counter: 2 };

        video_io.lock().set(0xFF40, 0x80);
        vdu_counter.tick();

        assert!(matches!(vdu_counter, VDUCounter::LCDOn { .. }));
    }

    #[test]
    fn lcdon_does_not_turn_to_lcdoff_when_lcdc_changes_on_even_tick () {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        video_io.lock().set(0xFF40, 0x00);
        let mut vdu_counter =VDUCounter::LCDOff { video_io: video_io.clone(), generic_frame_counter: 1 };

        video_io.lock().set(0xFF40, 0x80);
        vdu_counter.tick();

        assert!(matches!(vdu_counter, VDUCounter::LCDOff { .. }));
    }

    #[test]
    fn lcdcon_tick_on_even_sends_cpu_clock() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x01); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 2 };

        let events = vdu_counter.tick();

        assert!(matches!(events[0], ClockEvent::CPUClock));
    }


    #[test]
    fn lcdcon_tick_on_odd_sends_no_cpu_clock() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x01); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 1 };

        let events = vdu_counter.tick();

        assert_eq!(0, events.len());
    }

    #[test]
    fn lcdcon_tick_sends_draw_line_event_during_frame() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x01); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 0 };

        let events = vdu_counter.tick();

        assert!(matches!(events[1], ClockEvent::DrawLine));
    }

    #[test]
    fn lcdcon_tick_sends_vblank_interrupt_on_ly_90() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x8F); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 0 };

        let events = vdu_counter.tick();

        assert!(matches!(events[1], ClockEvent::VBlankInterrupt));
    }
    #[test]
    fn lcdcon_tick_sends_send_frame_on_ly_90() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x8F); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 0 };

        let events = vdu_counter.tick();

        assert!(matches!(events[2], ClockEvent::SendFrame));
    }

    #[test]
    fn lcdcon_tick_sends_no_draw_events_during_vblank() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x92); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 0 };

        let events = vdu_counter.tick();

        assert_eq!(1, events.len());
    }

    #[test]
    fn lcdcon_tick_resets_to_ly_0_at_ly_98() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x98); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 0 };

        vdu_counter.tick();

        assert_eq!(0, video_io.lock().get_ly());
    }

    #[test]
    fn lcdcon_tick_sends_draw_line_event_on_reset_from_ly_98() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x80);
        video_io.lock().set_ly(0x98); //generic mid frame

        let mut vdu_counter = VDUCounter::LCDOn { video_io: video_io.clone(), line_counter: 0 };

        let events = vdu_counter.tick();

        assert!(matches!(events[1], ClockEvent::DrawLine));
    }

    #[test]
    fn lcdoff_tick_on_even_sends_cpu_clock() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x00);

        let mut vdu_counter = VDUCounter::LCDOff { video_io: video_io.clone(), generic_frame_counter: 2 };

        let events = vdu_counter.tick();

        assert!(matches!(events[0], ClockEvent::CPUClock));
    }

    #[test]
    fn lcdoff_tick_on_odd_does_not_send_clock() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x00);

        let mut vdu_counter = VDUCounter::LCDOff { video_io: video_io.clone(), generic_frame_counter: 1 };

        let events = vdu_counter.tick();

        assert_eq!(0, events.len());
    }

    #[test]
    fn lcdoff_sends_frame_when_counter_is_0() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        video_io.lock().set(0xFF40, 0x00);

        let mut vdu_counter = VDUCounter::LCDOff { video_io: video_io.clone(), generic_frame_counter: 0 };

        let events = vdu_counter.tick();

        assert!(matches!(events[1], ClockEvent::SendFrame));
    }
}
