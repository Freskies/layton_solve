pub mod al_parcheggio;
pub mod dosa_il_liquido;
pub mod il_sogno_di_un_verme;
pub mod pallina_fuori;
pub mod salto_del_cavallo;

fn main() {
	let enigma: ENIGMI = ENIGMI::DosaLAcqua;
	println!("Start solving...");
	match enigma {
		ENIGMI::AlParcheggio => al_parcheggio::solve(),
		ENIGMI::DosaIlSucco => dosa_il_liquido::solve_succo(),
		ENIGMI::DosaIlLatte => dosa_il_liquido::solve_latte(),
		ENIGMI::PallinaFuori1 => pallina_fuori::solve(),
		ENIGMI::DosaLAcqua => dosa_il_liquido::solve_acqua(),
		ENIGMI::IlSognoDiUnVerme => il_sogno_di_un_verme::solve(),
		ENIGMI::SaltoDelCavallo1 => salto_del_cavallo::solve_1(),
		ENIGMI::SaltoDelCavallo2 => salto_del_cavallo::solve_2(),
		ENIGMI::SaltoDelCavallo3 => salto_del_cavallo::solve_3(),
		ENIGMI::SaltoDelCavallo4 => salto_del_cavallo::solve_4(),
	}
}

#[allow(unused)]
enum ENIGMI {
	// -- paese dei misteri --
	AlParcheggio,     // 019 - TODO
	DosaIlSucco,      // 023
	DosaIlLatte,      // 024
	PallinaFuori1,    // 058 - TODO
	DosaLAcqua,       // 078
	IlSognoDiUnVerme, // 107
	// -- scrigno di pandora --
	SaltoDelCavallo1,
	SaltoDelCavallo2,
	SaltoDelCavallo3,
	SaltoDelCavallo4,
}
