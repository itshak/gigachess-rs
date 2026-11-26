| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.989x | 1.475x |
| materialize | 0.962x | 0.897x | 1.446x |
| perft_bulk_d4 | 1.003x | 0.990x | 1.275x |
| perft_full_d3 | 1.015x | 1.003x | 1.604x |
| fen_parse | 0.975x | 0.952x | 1.025x |
| fen_write_native_ep | 1.002x | 0.969x | 1.943x |
| snapshot | 1.826x | 1.765x | 1.150x |
| san_parse_all | 0.979x | 0.966x | 1.784x |
| san_render_native | 0.986x | 0.906x | 2.116x |
| apply_children_full | 0.997x | 0.992x | 1.040x |
| make_unmake_full | 1.008x | 0.997x | 1.147x |
| startpos | 0.976x | 0.976x | 0.831x |
| chess960_materialize | 1.010x | 0.996x | 1.381x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.381x |
| polyglot_key | 0.990x | 0.989x | 41.263x |
| polyglot_recompute | 1.009x | 1.007x | 1.054x |
| repetition_key | 0.987x | 0.925x | 25.393x |
| corpus_count | 0.995x | 0.995x | 1.375x |
| corpus_materialize | 1.009x | 1.003x | 1.116x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.589x |
| replay_trusted_native | 0.937x | 0.880x | 0.663x |
| replay_checked_native | 0.985x | 0.981x | 1.051x |

Cold first legal move list: 0.010 ms -> 0.010 ms (0.94x).
