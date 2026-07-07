pub mod al_parcheggio;
pub mod da_mele_ad_arance;
pub mod dosa_il_liquido;
pub mod il_sogno_di_un_verme;
pub mod klotski;
pub mod salto_del_cavallo;

fn main() {
	let enigma: ENIGMI = ENIGMI::FuggiPrincipessa1;
	println!("Start solving...");
	match enigma {
		ENIGMI::AlParcheggio => al_parcheggio::solve(),
		ENIGMI::DosaIlSucco => dosa_il_liquido::solve_succo(),
		ENIGMI::DosaIlLatte => dosa_il_liquido::solve_latte(),
		ENIGMI::PallinaFuori1 => klotski::solve_pallina_fuori_1(),
		ENIGMI::PallinaFuori2 => klotski::solve_pallina_fuori_2(),
		ENIGMI::PallinaFuori4 => klotski::solve_pallina_fuori_4(),
		ENIGMI::DosaLAcqua => dosa_il_liquido::solve_acqua(),
		ENIGMI::DaMeleAdArance => da_mele_ad_arance::solve(),
		ENIGMI::FuggiPrincipessa1 => klotski::solve_fuggi_principessa_1(),
		ENIGMI::IlSognoDiUnVerme => il_sogno_di_un_verme::solve(),
		ENIGMI::Le4Palline => klotski::solve_le_4_palline(),
		ENIGMI::SaltoDelCavallo1 => salto_del_cavallo::solve_1(),
		ENIGMI::SaltoDelCavallo2 => salto_del_cavallo::solve_2(),
		ENIGMI::SaltoDelCavallo3 => salto_del_cavallo::solve_3(),
		ENIGMI::SaltoDelCavallo4 => salto_del_cavallo::solve_4(),
	}
}

#[allow(unused)]
enum ENIGMI {
	// -- paese dei misteri --
	AlParcheggio,      // 019 - TODO
	DosaIlSucco,       // 023
	DosaIlLatte,       // 024
	PallinaFuori1,     // 058
	PallinaFuori2,     // 090
	PallinaFuori4,     // 094
	DosaLAcqua,        // 078
	DaMeleAdArance,    // 079 - TODO
	FuggiPrincipessa1, // 097
	IlSognoDiUnVerme,  // 107
	Le4Palline,        // 129
	// -- scrigno di pandora --
	SaltoDelCavallo1,
	SaltoDelCavallo2,
	SaltoDelCavallo3,
	SaltoDelCavallo4,
}
