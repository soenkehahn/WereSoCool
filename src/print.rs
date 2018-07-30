extern crate weresocool;
use weresocool::compositions::song_2::generate_composition;
use weresocool::write_output_buffer::write_composition_to_wav;

fn main() {
    println!("{}", "\n ***** WereSoCool __!Now In Stereo!__ ****** \n ");

    write_composition_to_wav(generate_composition);

    println!("{}", "\n ***** WereSoFinishedWritingTheWavFile ****** \n ");
}
