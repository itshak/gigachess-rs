| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.967x | 0.664x | 1.260x |
| materialize | 1.049x | 0.864x | 1.096x |
| perft_bulk_d4 | 0.971x | 0.855x | 1.092x |
| perft_full_d3 | 0.954x | 0.754x | 1.362x |
| fen_parse | 0.975x | 0.859x | 1.275x |
| fen_write_native_ep | 0.961x | 0.880x | 1.717x |
| snapshot | 1.093x | 0.733x | 0.930x |
| san_parse_all | 0.932x | 0.871x | 1.882x |
| san_render_native | 0.967x | 0.802x | 2.739x |
| apply_children_full | 0.943x | 0.856x | 1.057x |
| make_unmake_full | 0.942x | 0.851x | 1.214x |
| startpos | 1.025x | 1.025x | 1.170x |
| chess960_materialize | 0.826x | 0.786x | 1.526x |
| chess960_perft_d3 | 0.882x | 0.776x | 1.156x |
| polyglot_key | 0.996x | 0.877x | 47.389x |
| polyglot_recompute | 0.756x | 0.602x | 0.912x |
| repetition_key | 1.085x | 0.866x | 20.579x |
| corpus_count | 0.713x | 0.580x | 0.972x |
| corpus_materialize | 0.878x | 0.811x | 1.022x |
| corpus_uci_sorted | 0.892x | 0.881x | 1.300x |
| replay_trusted_native | 1.070x | 0.940x | 0.607x |
| replay_checked_native | 0.958x | 0.871x | 1.054x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.82x).
