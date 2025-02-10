# Rust FRDM-HB2001-EVM control with KL25Z

Rust implementation of basic control for the FRDM-HB2001-EVM Evaluation Kit. The program implements basic SPI and PWM features for the KL25Z processor and uses them to command the MC33HB2001 H-Bridge IC. *This project does not implement any other features, and is not a functional board support crate*.

This project is designed to be used with probe-rs, however due to the lack of support for Kinetis processors within probe-rs the project includes a [linker script](frdm-kl25-hal/linker.x) and a [processor description](mkl25z4-pac/KLxx_Series.yaml). The linker script in particular is important as Kinetis MCUs have flash protection bits that can permanently disable access if mismanaged, therefore careful avoidance of this area of flash when programming is critical.

This project is unmaintained and will almost certainly not be developed further, but it could provide a useful starting point if someone was motivated to write a proper board support crate.

