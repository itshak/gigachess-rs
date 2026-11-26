| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.996x | 1.386x |
| materialize | 1.062x | 1.009x | 1.164x |
| perft_bulk_d4 | 1.015x | 1.005x | 1.334x |
| perft_full_d3 | 1.021x | 0.988x | 0.667x |
| fen_parse | 0.948x | 0.905x | 1.105x |
| fen_write_native_ep | 0.992x | 0.974x | 1.230x |
| snapshot | 1.230x | 1.218x | 1.246x |
| san_parse_all | 1.004x | 0.991x | 2.009x |
| san_render_native | 1.012x | 0.984x | 2.260x |
| apply_children_full | 0.854x | 0.829x | 0.958x |
| make_unmake_full | 0.997x | 0.975x | 0.970x |
| startpos | 0.912x | 0.912x | 0.998x |
| chess960_materialize | 1.001x | 0.994x | 1.511x |
| chess960_perft_d3 | 0.998x | 0.996x | 1.442x |
| polyglot_key | 0.991x | 0.980x | 20.818x |
| polyglot_recompute | 1.034x | 1.029x | 1.033x |
| repetition_key | 0.931x | 0.881x | 17.820x |
| corpus_count | 0.989x | 0.986x | 1.398x |
| corpus_materialize | 1.015x | 1.012x | 1.295x |
| corpus_uci_sorted | 0.992x | 0.985x | 0.968x |
| replay_trusted_native | 0.990x | 0.956x | 0.617x |
| replay_checked_native | 1.018x | 1.012x | 0.966x |

Cold first legal move list: 0.010 ms -> 0.010 ms (0.95x).
