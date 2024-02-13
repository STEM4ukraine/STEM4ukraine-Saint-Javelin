ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAHB;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAHC; loclib_name=3mmLED_backplane;
						li:objects {
						}
						ha:attrib {
							footprint=3mmLEDbackplane.lht
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=QH92z994CgKDCswi6t8AAAB1; loclib_name=3mmLED_pth;
						li:objects {
						}
						ha:attrib {
							footprint=LED3
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=G/87KL2bU4Z6G59Z1cgAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.12 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAA/; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=148000; y=72000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAABA; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAABB; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=5000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R2
					role=symbol
					value=220R
				}
			}
			ha:group.14 {
				uuid=G/87KL2bU4Z6G59Z1cgAAABL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=148000; y=52000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAABM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAABN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=5000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R3
					role=symbol
					value=220R
				}
			}
			ha:group.44 {
				uuid=G/87KL2bU4Z6G59Z1cgAAABy; src_uuid=G/87KL2bU4Z6G59Z1cgAAABa;
				x=200000; y=72000; rot=36.870000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAABz; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAB0; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=16000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=8000; y1=4000; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=12000; y1=0; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=8000; y1=4000; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=12000; y1=4000; x2=12000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=10000; y1=8000; x2=12000; y2=11000; stroke=sym-decor; }
					ha:line.10 { x1=12000; y1=11000; x2=11000; y2=10000; stroke=sym-decor; }
					ha:line.11 { x1=12000; y1=11000; x2=11483; y2=9545; stroke=sym-decor; }
					ha:line.12 { x1=8000; y1=7000; x2=10000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=10000; y1=10000; x2=10000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=9697; y1=6354; x2=11697; y2=9354; stroke=sym-decor; }
					ha:line.15 { x1=11697; y1=9354; x2=10697; y2=8354; stroke=sym-decor; }
					ha:line.16 { x1=11697; y1=9354; x2=11180; y2=7899; stroke=sym-decor; }
					ha:line.17 { x1=7697; y1=5354; x2=9697; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=9697; y1=8354; x2=9697; y2=6354; stroke=sym-decor; }
					ha:text.19 { x1=0; y1=4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					devmap=3mmLED_pth
					name=LED4
					role=symbol
				}
			}
			ha:group.55 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACj; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=148000; y=92000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAACk; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAACl; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=5000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R1
					role=symbol
					value=220R
				}
			}
			ha:group.56 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACm;
				li:objects {
					ha:line.1 { x1=168000; y1=52000; x2=224000; y2=52000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.58 {
				li:conn {
					/2/14/1/1
					/2/56/1
				}
			}
			ha:group.65 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACo;
				li:objects {
					ha:line.2 { x1=168000; y1=72000; x2=224000; y2=72000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.67 {
				li:conn {
					/2/12/1/1
					/2/65/2
				}
			}
			ha:group.68 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACp;
				li:objects {
					ha:line.3 { x1=168000; y1=92000; x2=224000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.69 {
				li:conn {
					/2/55/1/1
					/2/68/3
				}
			}
			ha:group.71 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACq;
				li:objects {
					ha:line.3 { x1=192000; y1=64000; x2=192000; y2=116000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.75 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACr;
				li:objects {
					ha:line.3 { x1=216000; y1=84000; x2=216000; y2=116000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.79 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACs;
				li:objects {
					ha:line.1 { x1=240000; y1=64000; x2=240000; y2=116000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.81 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACx; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=192000; y=116000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAACy; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=PB1
					role=symbol
				}
			}
			ha:connection.82 {
				li:conn {
					/2/81/1/1
					/2/71/3
				}
			}
			ha:group.83 {
				uuid=G/87KL2bU4Z6G59Z1cgAAACz; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=216000; y=116000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAC0; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=PB2
					role=symbol
				}
			}
			ha:connection.84 {
				li:conn {
					/2/83/1/1
					/2/75/3
				}
			}
			ha:group.85 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAC1; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=240000; y=116000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAC2; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=PB3
					role=symbol
				}
			}
			ha:connection.86 {
				li:conn {
					/2/85/1/1
					/2/79/1
				}
			}
			ha:group.87 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAC3; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=140000; y=92000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAC4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=PB0
					role=symbol
				}
			}
			ha:group.88 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAC5; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=140000; y=72000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAC6; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=PB1
					role=symbol
				}
			}
			ha:group.89 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAC7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=140000; y=52000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAC8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=PB2
					role=symbol
				}
			}
			ha:group.91 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAC/;
				li:objects {
					ha:line.1 { x1=148000; y1=92000; x2=140000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.92 {
				li:conn {
					/2/91/1
					/2/55/2/1
				}
			}
			ha:connection.93 {
				li:conn {
					/2/91/1
					/2/87/1/1
				}
			}
			ha:group.94 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADA;
				li:objects {
					ha:line.1 { x1=140000; y1=72000; x2=148000; y2=72000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.95 {
				li:conn {
					/2/94/1
					/2/12/2/1
				}
			}
			ha:connection.96 {
				li:conn {
					/2/94/1
					/2/88/1/1
				}
			}
			ha:group.97 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADB;
				li:objects {
					ha:line.1 { x1=148000; y1=52000; x2=140000; y2=52000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.98 {
				li:conn {
					/2/97/1
					/2/89/1/1
				}
			}
			ha:connection.99 {
				li:conn {
					/2/97/1
					/2/14/2/1
				}
			}
			ha:group.103 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADV; src_uuid=/iiShtebwvwwWnNJ68YAAAAJ;
				x=48000; y=80000;
				li:objects {
					ha:text.1 { x1=-3000; y1=25000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=28000; }
							ha:line { x1=0; y1=28000; x2=16000; y2=28000; }
							ha:line { x1=16000; y1=28000; x2=16000; y2=0; }
							ha:line { x1=16000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADW; src_uuid=/iiShtebwvwwWnNJ68YAAAAB;
						x=16000; y=24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB0
							pinnum=5
							role=terminal
						}
					}
					ha:group.4 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADX; src_uuid=/iiShtebwvwwWnNJ68YAAAAC;
						x=16000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB1
							pinnum=6
							role=terminal
						}
					}
					ha:group.5 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADY; src_uuid=/iiShtebwvwwWnNJ68YAAAAD;
						x=16000; y=16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB2
							pinnum=7
							role=terminal
						}
					}
					ha:group.6 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADZ; src_uuid=/iiShtebwvwwWnNJ68YAAAAE;
						x=16000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB3
							pinnum=2
							role=terminal
						}
					}
					ha:group.7 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADa; src_uuid=/iiShtebwvwwWnNJ68YAAAAF;
						x=16000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB4
							pinnum=3
							role=terminal
						}
					}
					ha:group.8 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADb; src_uuid=/iiShtebwvwwWnNJ68YAAAAG;
						x=16000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB5
							pinnum=1
							role=terminal
						}
					}
					ha:group.9 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADc; src_uuid=/iiShtebwvwwWnNJ68YAAAAH;
						x=8000; y=28000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=8
							role=terminal
						}
					}
					ha:group.10 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADd; src_uuid=/iiShtebwvwwWnNJ68YAAAAI;
						x=8000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=4
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=dip(8)
					name=U1
					role=symbol
				}
			}
			ha:group.104 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADe;
				x=-48000; y=12000;
				li:objects {
					ha:line.1 { x1=116000; y1=92000; x2=124000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					name=PB0
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.106 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADf;
				x=-48000; y=12000;
				li:objects {
					ha:line.1 { x1=116000; y1=88000; x2=124000; y2=88000; stroke=wire; }
				}
				ha:attrib {
					name=PB1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.108 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADg;
				x=-48000; y=12000;
				li:objects {
					ha:line.1 { x1=116000; y1=84000; x2=124000; y2=84000; stroke=wire; }
				}
				ha:attrib {
					name=PB2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.110 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADh;
				x=-48000; y=12000;
				li:objects {
					ha:line.1 { x1=116000; y1=80000; x2=124000; y2=80000; stroke=wire; }
				}
				ha:attrib {
					name=PB3
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.116 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADm; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=56000; y=72000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADn; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.117 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADs; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=56000; y=116000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAADt; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.118 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADu;
				x=16000; y=-36000;
				li:objects {
					ha:line.1 { x1=40000; y1=152000; x2=40000; y2=148000; stroke=wire; }
					ha:line.3 { x1=16000; y1=148000; x2=16000; y2=140000; stroke=wire; }
					ha:line.4 { x1=16000; y1=148000; x2=72000; y2=148000; stroke=wire; }
					ha:line.7 { x1=72000; y1=148000; x2=72000; y2=144000; stroke=wire; }
					ha:line.8 { x1=40000; y1=148000; x2=40000; y2=148000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.121 {
				uuid=G/87KL2bU4Z6G59Z1cgAAADv;
				x=16000; y=-36000;
				li:objects {
					ha:line.1 { x1=40000; y1=112000; x2=40000; y2=108000; stroke=wire; }
					ha:line.2 { x1=40000; y1=112000; x2=16000; y2=112000; stroke=wire; }
					ha:line.3 { x1=16000; y1=112000; x2=16000; y2=120000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.134 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAFA; src_uuid=G/87KL2bU4Z6G59Z1cgAAAE3;
				x=100000; y=84000;
				li:objects {
					ha:text.1 { x1=2000; y1=-5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFB; src_uuid=G/87KL2bU4Z6G59Z1cgAAAE4;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:line.3 { x1=0; y1=0; x2=1200; y2=0; stroke=sym-decor; }
					ha:arc.4 { cx=1600; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:group.5 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFC; src_uuid=G/87KL2bU4Z6G59Z1cgAAAE5;
						x=8000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:line.6 { x1=6800; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:arc.7 { cx=6400; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.8 { x1=0; y1=0; x2=0; y2=0; stroke=sym-decor; }
					ha:line.9 { x1=0; y1=0; x2=1200; y2=0; stroke=sym-decor; }
					ha:arc.10 { cx=1600; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.11 { x1=1200; y1=3600; x2=6800; y2=3600; stroke=sym-decor; }
					ha:line.12 { x1=1600; y1=8000; x2=6400; y2=8000; stroke=sym-decor; }
					ha:line.13 { x1=1600; y1=8000; x2=1600; y2=7200; stroke=sym-decor; }
					ha:line.14 { x1=6400; y1=8000; x2=6400; y2=7200; stroke=sym-decor; }
					ha:line.15 { x1=4000; y1=3600; x2=4000; y2=8000; stroke=sym-decor; }
				}
				ha:attrib {
					footprint=switchMomentaryPCB.lht
					name=SW1
					role=symbol
				}
			}
			ha:group.135 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAFJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=32000; y=104000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=C200
					name=C1
					role=symbol
					value=100n
				}
			}
			ha:connection.143 {
				li:conn {
					/2/103/9/1
					/2/118/4
					/2/118/1
					/2/118/8
				}
			}
			ha:connection.144 {
				li:conn {
					/2/103/10/1
					/2/121/2
					/2/121/1
				}
			}
			ha:group.148 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAFP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=88000; y=108000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFR; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R4
					role=symbol
					value=10k
				}
			}
			ha:group.149 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAFS; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=120000; y=80000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFT; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFU; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R5
					role=symbol
					value=47k
				}
			}
			ha:group.154 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAFW;
				x=-12000; y=0;
				li:objects {
					ha:line.2 { x1=100000; y1=88000; x2=100000; y2=84000; stroke=wire; }
					ha:line.4 { x1=80000; y1=84000; x2=108000; y2=84000; stroke=wire; }
					ha:line.7 { x1=108000; y1=84000; x2=108000; y2=76000; stroke=wire; }
					ha:line.9 { x1=108000; y1=76000; x2=114000; y2=76000; stroke=wire; }
					ha:line.10 { x1=100000; y1=84000; x2=100000; y2=84000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.158 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAFX;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=124000; y1=56000; x2=124000; y2=60000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.160 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAFa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=120000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAFb; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.167 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAGj; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=128000; y=104000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAGk; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.168 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAGn; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=128000; y=116000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAGo; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.178 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAGv; src_uuid=G/87KL2bU4Z6G59Z1cgAAABa;
				x=176000; y=52000; rot=36.870000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAGw; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAGx; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=16000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=8000; y1=4000; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=12000; y1=0; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=8000; y1=4000; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=12000; y1=4000; x2=12000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=10000; y1=8000; x2=12000; y2=11000; stroke=sym-decor; }
					ha:line.10 { x1=12000; y1=11000; x2=11000; y2=10000; stroke=sym-decor; }
					ha:line.11 { x1=12000; y1=11000; x2=11483; y2=9545; stroke=sym-decor; }
					ha:line.12 { x1=8000; y1=7000; x2=10000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=10000; y1=10000; x2=10000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=9697; y1=6354; x2=11697; y2=9354; stroke=sym-decor; }
					ha:line.15 { x1=11697; y1=9354; x2=10697; y2=8354; stroke=sym-decor; }
					ha:line.16 { x1=11697; y1=9354; x2=11180; y2=7899; stroke=sym-decor; }
					ha:line.17 { x1=7697; y1=5354; x2=9697; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=9697; y1=8354; x2=9697; y2=6354; stroke=sym-decor; }
					ha:text.19 { x1=0; y1=4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					devmap=3mmLED_pth
					name=LED6
					role=symbol
				}
			}
			ha:connection.179 {
				li:conn {
					/2/178/1/1
					/2/71/3
				}
			}
			ha:connection.180 {
				li:conn {
					/2/178/2/1
					/2/56/1
				}
			}
			ha:group.181 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAGy; src_uuid=G/87KL2bU4Z6G59Z1cgAAABa;
				x=224000; y=52000; rot=36.870000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAGz; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG0; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=16000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=8000; y1=4000; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=12000; y1=0; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=8000; y1=4000; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=12000; y1=4000; x2=12000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=10000; y1=8000; x2=12000; y2=11000; stroke=sym-decor; }
					ha:line.10 { x1=12000; y1=11000; x2=11000; y2=10000; stroke=sym-decor; }
					ha:line.11 { x1=12000; y1=11000; x2=11483; y2=9545; stroke=sym-decor; }
					ha:line.12 { x1=8000; y1=7000; x2=10000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=10000; y1=10000; x2=10000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=9697; y1=6354; x2=11697; y2=9354; stroke=sym-decor; }
					ha:line.15 { x1=11697; y1=9354; x2=10697; y2=8354; stroke=sym-decor; }
					ha:line.16 { x1=11697; y1=9354; x2=11180; y2=7899; stroke=sym-decor; }
					ha:line.17 { x1=7697; y1=5354; x2=9697; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=9697; y1=8354; x2=9697; y2=6354; stroke=sym-decor; }
					ha:text.19 { x1=0; y1=4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					devmap=3mmLED_pth
					name=LED7
					role=symbol
				}
			}
			ha:group.184 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAG1; src_uuid=G/87KL2bU4Z6G59Z1cgAAABa;
				x=224000; y=92000; rot=36.870000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG2; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG3; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=16000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=8000; y1=4000; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=12000; y1=0; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=8000; y1=4000; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=12000; y1=4000; x2=12000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=10000; y1=8000; x2=12000; y2=11000; stroke=sym-decor; }
					ha:line.10 { x1=12000; y1=11000; x2=11000; y2=10000; stroke=sym-decor; }
					ha:line.11 { x1=12000; y1=11000; x2=11483; y2=9545; stroke=sym-decor; }
					ha:line.12 { x1=8000; y1=7000; x2=10000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=10000; y1=10000; x2=10000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=9697; y1=6354; x2=11697; y2=9354; stroke=sym-decor; }
					ha:line.15 { x1=11697; y1=9354; x2=10697; y2=8354; stroke=sym-decor; }
					ha:line.16 { x1=11697; y1=9354; x2=11180; y2=7899; stroke=sym-decor; }
					ha:line.17 { x1=7697; y1=5354; x2=9697; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=9697; y1=8354; x2=9697; y2=6354; stroke=sym-decor; }
					ha:text.19 { x1=0; y1=4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					devmap=3mmLED_pth
					name=LED3
					role=symbol
				}
			}
			ha:group.187 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAG4; src_uuid=G/87KL2bU4Z6G59Z1cgAAABa;
				x=224000; y=72000; rot=36.870000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG5; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG6; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=16000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=8000; y1=4000; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=12000; y1=0; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=8000; y1=4000; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=12000; y1=4000; x2=12000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=10000; y1=8000; x2=12000; y2=11000; stroke=sym-decor; }
					ha:line.10 { x1=12000; y1=11000; x2=11000; y2=10000; stroke=sym-decor; }
					ha:line.11 { x1=12000; y1=11000; x2=11483; y2=9545; stroke=sym-decor; }
					ha:line.12 { x1=8000; y1=7000; x2=10000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=10000; y1=10000; x2=10000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=9697; y1=6354; x2=11697; y2=9354; stroke=sym-decor; }
					ha:line.15 { x1=11697; y1=9354; x2=10697; y2=8354; stroke=sym-decor; }
					ha:line.16 { x1=11697; y1=9354; x2=11180; y2=7899; stroke=sym-decor; }
					ha:line.17 { x1=7697; y1=5354; x2=9697; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=9697; y1=8354; x2=9697; y2=6354; stroke=sym-decor; }
					ha:text.19 { x1=0; y1=4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					devmap=3mmLED_pth
					name=LED5
					role=symbol
				}
			}
			ha:group.190 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAG7; src_uuid=G/87KL2bU4Z6G59Z1cgAAABa;
				x=200000; y=92000; rot=36.870000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG8; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG9; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=16000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=8000; y1=4000; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=12000; y1=0; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=8000; y1=4000; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=12000; y1=4000; x2=12000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=10000; y1=8000; x2=12000; y2=11000; stroke=sym-decor; }
					ha:line.10 { x1=12000; y1=11000; x2=11000; y2=10000; stroke=sym-decor; }
					ha:line.11 { x1=12000; y1=11000; x2=11483; y2=9545; stroke=sym-decor; }
					ha:line.12 { x1=8000; y1=7000; x2=10000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=10000; y1=10000; x2=10000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=9697; y1=6354; x2=11697; y2=9354; stroke=sym-decor; }
					ha:line.15 { x1=11697; y1=9354; x2=10697; y2=8354; stroke=sym-decor; }
					ha:line.16 { x1=11697; y1=9354; x2=11180; y2=7899; stroke=sym-decor; }
					ha:line.17 { x1=7697; y1=5354; x2=9697; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=9697; y1=8354; x2=9697; y2=6354; stroke=sym-decor; }
					ha:text.19 { x1=0; y1=4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					devmap=3mmLED_pth
					name=LED2
					role=symbol
				}
			}
			ha:group.193 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAG+; src_uuid=G/87KL2bU4Z6G59Z1cgAAABa;
				x=176000; y=92000; rot=36.870000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAG/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAHA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:line.3 { x1=16000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=8000; y1=4000; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=12000; y1=0; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=8000; y1=4000; x2=8000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=12000; y1=4000; x2=12000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=10000; y1=8000; x2=12000; y2=11000; stroke=sym-decor; }
					ha:line.10 { x1=12000; y1=11000; x2=11000; y2=10000; stroke=sym-decor; }
					ha:line.11 { x1=12000; y1=11000; x2=11483; y2=9545; stroke=sym-decor; }
					ha:line.12 { x1=8000; y1=7000; x2=10000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=10000; y1=10000; x2=10000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=9697; y1=6354; x2=11697; y2=9354; stroke=sym-decor; }
					ha:line.15 { x1=11697; y1=9354; x2=10697; y2=8354; stroke=sym-decor; }
					ha:line.16 { x1=11697; y1=9354; x2=11180; y2=7899; stroke=sym-decor; }
					ha:line.17 { x1=7697; y1=5354; x2=9697; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=9697; y1=8354; x2=9697; y2=6354; stroke=sym-decor; }
					ha:text.19 { x1=0; y1=4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					devmap=3mmLED_pth
					name=LED1
					role=symbol
				}
			}
			ha:connection.195 {
				li:conn {
					/2/193/2/1
					/2/68/3
				}
			}
			ha:group.196 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAHG; src_uuid=w6A0A3kgnq7mqMH/npwAAAFa;
				x=80000; y=68000;
				li:objects {
					ha:arc.1 { cx=2000; cy=-4000; r=6000; sang=270.000000; dang=180.000000; stroke=sym-decor; }
					ha:line.2 { x1=2000; y1=2000; x2=0; y2=2000; stroke=sym-decor; }
					ha:line.3 { x1=0; y1=2000; x2=0; y2=-10000; stroke=sym-decor; }
					ha:line.4 { x1=0; y1=-10000; x2=2000; y2=-10000; stroke=sym-decor; }
					ha:group.5 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAHH; src_uuid=w6A0A3kgnq7mqMH/npwAAAFY;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAHI; src_uuid=w6A0A3kgnq7mqMH/npwAAAFZ;
						x=0; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:text.7 { x1=0; y1=2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					footprint=piezo_5mm_to_7.62mm.rf
					name=PIEZO
					role=symbol
				}
			}
			ha:group.197 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAHL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=72000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAHM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.199 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAHO;
				x=-12000; y=0;
				li:objects {
					ha:line.1 { x1=84000; y1=56000; x2=84000; y2=60000; stroke=wire; }
					ha:line.2 { x1=84000; y1=60000; x2=88000; y2=60000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.202 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAHP;
				x=-12000; y=0;
				li:objects {
					ha:line.1 { x1=88000; y1=68000; x2=84000; y2=68000; stroke=wire; }
					ha:line.2 { x1=84000; y1=68000; x2=84000; y2=88000; stroke=wire; }
					ha:line.3 { x1=84000; y1=88000; x2=80000; y2=88000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.205 {
				li:conn {
					/2/193/1/1
					/2/71/3
				}
			}
			ha:connection.206 {
				li:conn {
					/2/44/1/1
					/2/75/3
				}
			}
			ha:connection.207 {
				li:conn {
					/2/44/2/1
					/2/65/2
				}
			}
			ha:connection.208 {
				li:conn {
					/2/184/1/1
					/2/79/1
				}
			}
			ha:connection.210 {
				li:conn {
					/2/190/1/1
					/2/75/3
				}
			}
			ha:connection.211 {
				li:conn {
					/2/190/2/1
					/2/68/3
				}
			}
			ha:connection.212 {
				li:conn {
					/2/68/3
					/2/184/2/1
				}
			}
			ha:connection.213 {
				li:conn {
					/2/187/1/1
					/2/79/1
				}
			}
			ha:connection.214 {
				li:conn {
					/2/65/2
					/2/187/2/1
				}
			}
			ha:connection.215 {
				li:conn {
					/2/181/1/1
					/2/79/1
				}
			}
			ha:connection.216 {
				li:conn {
					/2/181/2/1
					/2/56/1
				}
			}
			ha:connection.217 {
				li:conn {
					/2/104/1
					/2/103/3/1
				}
			}
			ha:connection.218 {
				li:conn {
					/2/106/1
					/2/103/4/1
				}
			}
			ha:connection.219 {
				li:conn {
					/2/108/1
					/2/103/5/1
				}
			}
			ha:connection.220 {
				li:conn {
					/2/110/1
					/2/103/6/1
				}
			}
			ha:connection.221 {
				li:conn {
					/2/118/1
					/2/117/1/1
				}
			}
			ha:connection.222 {
				li:conn {
					/2/121/1
					/2/116/1/1
				}
			}
			ha:connection.224 {
				li:conn {
					/2/135/1/1
					/2/121/3
				}
			}
			ha:connection.225 {
				li:conn {
					/2/135/2/1
					/2/118/3
				}
			}
			ha:connection.226 {
				li:conn {
					/2/148/2/1
					/2/118/7
				}
			}
			ha:connection.229 {
				li:conn {
					/2/103/8/1
					/2/154/4
				}
			}
			ha:connection.230 {
				li:conn {
					/2/154/2
					/2/148/1/1
				}
			}
			ha:connection.237 {
				li:conn {
					/2/199/1
					/2/197/1/1
				}
			}
			ha:connection.238 {
				li:conn {
					/2/199/2
					/2/196/6/1
				}
			}
			ha:connection.239 {
				li:conn {
					/2/202/1
					/2/196/5/1
				}
			}
			ha:connection.240 {
				li:conn {
					/2/202/3
					/2/103/7/1
				}
			}
			ha:connection.242 {
				li:conn {
					/2/168/1/1
					/2/262/3
				}
			}
			ha:connection.244 {
				li:conn {
					/2/167/1/1
					/2/267/2
				}
			}
			ha:connection.246 {
				li:conn {
					/2/158/1
					/2/149/1/1
				}
			}
			ha:connection.247 {
				li:conn {
					/2/160/1/1
					/2/158/1
				}
			}
			ha:connection.248 {
				li:conn {
					/2/154/4
					/2/134/2/1
					/2/154/7
				}
			}
			ha:group.251 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAIR; src_uuid=G/87KL2bU4Z6G59Z1cgAAAIO;
				x=102000; y=72000; rot=270.000000;
				li:objects {
					ha:text.1 { x1=8000; y1=-2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAIS; src_uuid=G/87KL2bU4Z6G59Z1cgAAAIP;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAIT; src_uuid=G/87KL2bU4Z6G59Z1cgAAAIQ;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=EXTSW
					role=symbol
				}
			}
			ha:connection.252 {
				li:conn {
					/2/154/9
					/2/251/2/1
				}
			}
			ha:group.255 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAIV;
				li:objects {
					ha:line.1 { x1=112000; y1=76000; x2=112000; y2=84000; stroke=wire; }
					ha:line.2 { x1=120000; y1=84000; x2=120000; y2=80000; stroke=wire; }
					ha:line.3 { x1=112000; y1=84000; x2=120000; y2=84000; stroke=wire; }
					ha:line.4 { x1=106000; y1=76000; x2=112000; y2=76000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.256 {
				li:conn {
					/2/255/1
					/2/134/5/1
					/2/255/3
				}
			}
			ha:connection.257 {
				li:conn {
					/2/255/2
					/2/149/2/1
				}
			}
			ha:connection.258 {
				li:conn {
					/2/255/4
					/2/251/3/1
				}
			}
			ha:group.259 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAIZ; src_uuid=G/87KL2bU4Z6G59Z1cgAAAIO;
				x=137000; y=108000;
				li:objects {
					ha:text.1 { x1=0; y1=6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAIa; src_uuid=G/87KL2bU4Z6G59Z1cgAAAIP;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAIb; src_uuid=G/87KL2bU4Z6G59Z1cgAAAIQ;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=EXT5V
					role=symbol
				}
			}
			ha:group.262 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAIc;
				x=4000; y=0;
				li:objects {
					ha:line.1 { x1=129000; y1=112000; x2=129000; y2=116000; stroke=wire; }
					ha:line.3 { x1=116000; y1=116000; x2=129000; y2=116000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.264 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAJf; src_uuid=G/87KL2bU4Z6G59Z1cgAAAJY;
				x=116000; y=116000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=-1000; y1=-5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAJg; src_uuid=G/87KL2bU4Z6G59Z1cgAAAJZ;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAJh; src_uuid=G/87KL2bU4Z6G59Z1cgAAAJa;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAJi; src_uuid=G/87KL2bU4Z6G59Z1cgAAAJb;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAJj; src_uuid=G/87KL2bU4Z6G59Z1cgAAAJc;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAJk; src_uuid=G/87KL2bU4Z6G59Z1cgAAAJd;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=G/87KL2bU4Z6G59Z1cgAAAJl; src_uuid=G/87KL2bU4Z6G59Z1cgAAAJe;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=TypeBpthUSBvertical.lht
					name=USB1
					role=symbol
				}
			}
			ha:connection.266 {
				li:conn {
					/2/264/5/1
					/2/267/2
					/2/267/4
				}
			}
			ha:group.267 {
				uuid=G/87KL2bU4Z6G59Z1cgAAAJm;
				x=4000; y=0;
				li:objects {
					ha:line.2 { x1=116000; y1=104000; x2=129000; y2=104000; stroke=wire; }
					ha:line.3 { x1=129000; y1=104000; x2=129000; y2=108000; stroke=wire; }
					ha:line.4 { x1=116000; y1=96000; x2=116000; y2=104000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.268 {
				li:conn {
					/2/264/6/1
					/2/267/4
				}
			}
			ha:connection.269 {
				li:conn {
					/2/264/7/1
					/2/267/4
				}
			}
			ha:connection.270 {
				li:conn {
					/2/262/1
					/2/259/3/1
				}
			}
			ha:connection.271 {
				li:conn {
					/2/264/2/1
					/2/262/3
				}
			}
			ha:connection.272 {
				li:conn {
					/2/267/3
					/2/259/2/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 0
     grid = 1.0240 mm
    }
   }
  }
}
