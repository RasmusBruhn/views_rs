use winit;
use crate::view::{View, extent, children};

/// Defines a wrapper of a winit event loop which will deal with all the gui handling before passing events on to the user
#[derive(Debug)]
pub struct EventLoop<T: 'static> {
    /// The winit event loop to run
    event_loop: winit::event_loop::EventLoop<T>,
    /// The root view
    root: View,
}

impl EventLoop<()> {
    /// Creates a new basic event loop
    /// 
    /// # Parameters
    /// 
    /// root: The root view for the window
    pub fn new(root: View) -> Result<Self, children::ValidateError> {
        // Make sure the root is valid
        root.validate(&[])?;

        // Create an event loop
        let event_loop = winit::event_loop::EventLoop::new();

        Ok(Self { event_loop, root })
    }
}

impl<T: 'static> EventLoop<T> {
    /// Creates an event loop based on the winit event loop given
    /// 
    /// # Parameters
    /// 
    /// root: The root view for the window
    /// 
    /// event_loop: The winit event loop to run
    pub fn from_winit_loop(root: View, event_loop: winit::event_loop::EventLoop<T>) -> Result<Self, children::ValidateError> {
            // Make sure the root is valid
            root.validate(&[])?;

            Ok(Self { event_loop, root })
    }

    /// Returns a reference to the winit event loop to use it to build windows and other things
    pub fn get_winit_loop(&self) -> &winit::event_loop::EventLoop<T> {
        &self.event_loop
    }

    /// Returns a mutable reference to the winit event loop
    pub fn get_winit_loop_mut(&mut self) -> &mut winit::event_loop::EventLoop<T> {
        &mut self.event_loop
    }

    /// Starts the event loop
    /// 
    /// # Parameters
    /// 
    /// root: The root view for the window
    /// 
    /// event_handler: The event handler to run after the gui events has been handled
    pub fn run<F>(self, mut event_handler: F)
    where
        F: 'static + FnMut(winit::event::Event<'_, T>, &winit::event_loop::EventLoopWindowTarget<T>, &mut winit::event_loop::ControlFlow),
    {
        // Create the event handler
        let view_event_handler = move |event: winit::event::Event<'_, T>, window_target: &winit::event_loop::EventLoopWindowTarget<T>, control_flow: &mut winit::event_loop::ControlFlow| {
            match event {
                // All events are done and the root must be updated
                winit::event::Event::MainEventsCleared => {
                    self.root.update();
                }

                _ => (),
            }

            // Run the user events
            event_handler(event, window_target, control_flow);
        };

        self.event_loop.run(view_event_handler)
    }
}