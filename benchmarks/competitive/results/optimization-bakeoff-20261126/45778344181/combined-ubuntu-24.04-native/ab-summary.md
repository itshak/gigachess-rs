| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.995x | 1.407x |
| materialize | 1.081x | 1.033x | 1.507x |
| perft_bulk_d4 | 0.924x | 0.882x | 1.119x |
| perft_full_d3 | 1.172x | 1.068x | 1.481x |
| fen_parse | 1.037x | 0.956x | 0.998x |
| fen_write_native_ep | 0.945x | 0.923x | 1.320x |
| snapshot | 0.671x | 0.667x | 0.871x |
| san_parse_all | 0.988x | 0.969x | 1.856x |
| san_render_native | 0.959x | 0.927x | 2.055x |
| apply_children_full | 0.673x | 0.653x | 0.759x |
| make_unmake_full | 0.872x | 0.828x | 1.002x |
| startpos | 0.813x | 0.813x | 1.212x |
| chess960_materialize | 1.011x | 1.006x | 1.459x |
| chess960_perft_d3 | 0.910x | 0.877x | 1.268x |
| polyglot_key | 0.985x | 0.981x | 38.324x |
| polyglot_recompute | 1.041x | 1.037x | 1.012x |
| repetition_key | 1.068x | 0.894x | 20.607x |
| corpus_count | 1.008x | 1.004x | 1.331x |
| corpus_materialize | 1.020x | 1.019x | 1.091x |
| corpus_uci_sorted | 0.988x | 0.987x | 1.558x |
| replay_trusted_native | 0.971x | 0.952x | 0.700x |
| replay_checked_native | 0.991x | 0.987x | 1.046x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.93x).
