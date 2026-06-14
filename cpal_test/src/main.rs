use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, StreamConfig};

fn main() {
    println!("=== ALVR/CPAL Windows Audio Diagnostic Tool ===");
    
    let host = cpal::default_host();
    println!("Using default audio host: {:?}", host.id());
    
    let devices = match host.devices() {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to query audio devices: {:?}", e);
            return;
        }
    };
    
    let target_config = StreamConfig {
        channels: 1, // 1 channel (Mono)
        sample_rate: cpal::SampleRate(48000), // 48000 Hz
        buffer_size: BufferSize::Default,
    };
    
    println!("\nTarget stream configuration ALVR is requesting:");
    println!(" - Channels: {}", target_config.channels);
    println!(" - Sample Rate: {} Hz", target_config.sample_rate.0);
    println!(" - Format: I16 (16-bit Signed Integer)");
    
    println!("\n=== Scanning Devices ===");
    for (idx, device) in devices.enumerate() {
        let name = device.name().unwrap_or_else(|_| "Unknown Device".to_string());
        println!("\n[{}] Device Name: {}", idx + 1, name);
        
        // Query supported output configs
        println!("  > Querying supported configurations (Output):");
        let mut supported_channels = Vec::new();
        if let Ok(configs) = device.supported_output_configs() {
            let mut count = 0;
            for config in configs {
                println!("    * Supported: channels={}, sample_rate={} Hz - {} Hz, format={:?}", 
                    config.channels(), 
                    config.min_sample_rate().0,
                    config.max_sample_rate().0,
                    config.sample_format()
                );
                supported_channels.push(config.channels());
                count += 1;
            }
            if count == 0 {
                println!("    (No supported output configurations reported)");
            }
        } else {
            println!("    (Failed to query supported output configurations)");
        }
        
        let default_config = match device.default_output_config() {
            Ok(c) => c,
            Err(e) => {
                println!("  > Failed to get default output config: {:?}", e);
                continue;
            }
        };
        println!("  > Default output config: channels={}, sample_rate={} Hz, format={:?}",
            default_config.channels(),
            default_config.sample_rate().0,
            default_config.sample_format()
        );

        let mut chosen_channels = 1;
        if !supported_channels.contains(&1) {
            chosen_channels = default_config.channels();
            println!("  > Mono (1 channel) output not supported by device. Falling back to default channels: {}", chosen_channels);
        } else {
            println!("  > Mono (1 channel) output is supported.");
        }

        let test_config = StreamConfig {
            channels: chosen_channels,
            sample_rate: cpal::SampleRate(48000),
            buffer_size: BufferSize::Default,
        };
        
        // Test attempting to build an output stream (plays microphone audio into the virtual cable)
        print!("  > Testing WASAPI build_output_stream (Play Mic -> CABLE Input) with {} channels... ", chosen_channels);
        let out_result = device.build_output_stream(
            &test_config,
            move |_data: &mut [i16], _: &cpal::OutputCallbackInfo| {},
            move |err| println!("Stream error: {:?}", err),
            None
        );
        
        match out_result {
            Ok(_stream) => {
                println!("SUCCESS! Output Stream built successfully.");
            }
            Err(e) => {
                println!("FAILED! Error Details: {:?}", e);
            }
        }
        
        // Test attempting to build an input stream (records game audio)
        print!("  > Testing WASAPI build_input_stream (Record Game Audio)... ");
        let in_result = device.build_input_stream(
            &test_config,
            move |_data: &[i16], _: &cpal::InputCallbackInfo| {},
            move |err| println!("Stream error: {:?}", err),
            None
        );
        
        match in_result {
            Ok(_stream) => {
                println!("SUCCESS! Input Stream built successfully.");
            }
            Err(e) => {
                println!("FAILED! Error Details: {:?}", e);
            }
        }
    }
}
