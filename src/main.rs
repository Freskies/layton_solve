pub mod al_parcheggio;
pub mod d33333;
pub mod dosa_il_liquido;
pub mod frittelle_impilate;
pub mod il_sogno_di_un_verme;
pub mod klotski;
pub mod lattine_e_barattoli;
pub mod mangia_la_pallina;
pub mod otto_carte;
pub mod salto_del_cavallo;
pub mod troppe_regine;

fn main() {
	let enigma: ENIGMI = ENIGMI::LaSferaBloccata;
	println!("Start solving...");
	match enigma {
		// paese dei misteri
		ENIGMI::AlParcheggio => al_parcheggio::solve(),
		ENIGMI::DosaIlSucco => dosa_il_liquido::solve_succo(),
		ENIGMI::DosaIlLatte => dosa_il_liquido::solve_latte(),
		ENIGMI::PallinaFuori1 => klotski::solve_pallina_fuori_1(),
		ENIGMI::PallinaFuori2 => klotski::solve_pallina_fuori_2(),
		ENIGMI::PallinaFuori4 => klotski::solve_pallina_fuori_4(),
		ENIGMI::DosaLAcqua => dosa_il_liquido::solve_acqua(),
		ENIGMI::FuggiPrincipessa1 => klotski::solve_fuggi_principessa_1(),
		ENIGMI::D33333 => d33333::solve(),
		ENIGMI::IlSognoDiUnVerme => il_sogno_di_un_verme::solve(),
		ENIGMI::PallinaFuori3 => klotski::solve_pallina_fuori_3(),
		ENIGMI::Le4Palline => klotski::solve_le_4_palline(),
		ENIGMI::TroppeRegine5 => troppe_regine::solve(),
		ENIGMI::FuggiPrincipessa2 => klotski::solve_fuggi_principessa_2(),
		ENIGMI::FuggiPrincipessa3 => klotski::solve_fuggi_principessa_3(),
		// scrigno di pandora
		ENIGMI::FrittelleImpilate1 => frittelle_impilate::solve_1(),
		ENIGMI::ChiETom => klotski::solve_chi_e_tom(),
		ENIGMI::LOraDellePulizie1 => klotski::solve_l_ora_delle_pulizie_1(),
		ENIGMI::RitiroBagnagli => klotski::solve_ritiro_bagagli(),
		ENIGMI::InvertiLeSfere => klotski::solve_inverti_le_sfere(),
		ENIGMI::FrittelleImpilate2 => frittelle_impilate::solve_2(),
		ENIGMI::FrittelleImpilate3 => frittelle_impilate::solve_3(),
		ENIGMI::LattineBarattoli1 => lattine_e_barattoli::solve_1(),
		ENIGMI::LattineBarattoli2 => lattine_e_barattoli::solve_2(),
		ENIGMI::MangiaLaPallina1 => mangia_la_pallina::solve_1(),
		ENIGMI::MangiaLaPallina2 => mangia_la_pallina::solve_2(),
		ENIGMI::MangiaLaPallina3 => mangia_la_pallina::solve_3(),
		ENIGMI::MangiaLaPallina4 => mangia_la_pallina::solve_4(),
		ENIGMI::MangiaLaPallina5 => mangia_la_pallina::solve_5(),
		ENIGMI::SaltoDelCavallo1 => salto_del_cavallo::solve_1(),
		ENIGMI::SaltoDelCavallo2 => salto_del_cavallo::solve_2(),
		ENIGMI::SaltoDelCavallo3 => salto_del_cavallo::solve_3(),
		ENIGMI::SaltoDelCavallo4 => salto_del_cavallo::solve_4(),
		ENIGMI::LOraDellePulizie2 => klotski::solve_l_ora_delle_pulizie_2(),
		ENIGMI::PrendiLaChiave => klotski::solve_prendi_la_chiave(),
		ENIGMI::MangiaLaPallina6 => mangia_la_pallina::solve_6(),
		ENIGMI::SuperFrittelle => frittelle_impilate::solve_s(),
		ENIGMI::IlLabirintoDiTasselli => il_sogno_di_un_verme::solve_labirinto_a_tasselli(),
		ENIGMI::OttoCarte => otto_carte::solve(),
		ENIGMI::LaSferaBloccata => klotski::solve_la_sfera_bloccata(),
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
	FuggiPrincipessa1, // 097
	D33333,            // 099
	IlSognoDiUnVerme,  // 107
	PallinaFuori3,     // 120
	Le4Palline,        // 129
	TroppeRegine5,     // 130
	FuggiPrincipessa2, // 132
	FuggiPrincipessa3, // 135
	// -- scrigno di pandora --
	FrittelleImpilate1,    // 006
	ChiETom,               // 015
	LOraDellePulizie1,     // 022
	RitiroBagnagli,        // 045
	InvertiLeSfere,        // 080
	FrittelleImpilate2,    // 083
	FrittelleImpilate3,    // 084
	LattineBarattoli1,     // 098
	LattineBarattoli2,     // 099
	MangiaLaPallina1,      // 101
	MangiaLaPallina2,      // 102
	MangiaLaPallina3,      // 103
	MangiaLaPallina4,      // 104
	MangiaLaPallina5,      // 105
	SaltoDelCavallo1,      // 106
	SaltoDelCavallo2,      // 107
	SaltoDelCavallo3,      // 108
	SaltoDelCavallo4,      // 109
	LOraDellePulizie2,     // 118
	PrendiLaChiave,        // 133
	MangiaLaPallina6,      // 141
	SuperFrittelle,        // 144
	IlLabirintoDiTasselli, // 147
	OttoCarte,             // 148
	LaSferaBloccata,       // 153
}
