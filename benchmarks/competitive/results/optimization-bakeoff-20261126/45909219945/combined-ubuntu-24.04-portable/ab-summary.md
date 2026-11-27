| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.944x | 1.315x |
| materialize | 0.922x | 0.886x | 1.636x |
| perft_bulk_d4 | 1.025x | 0.989x | 1.220x |
| perft_full_d3 | 0.994x | 0.987x | 1.321x |
| fen_parse | 0.996x | 0.958x | 1.196x |
| fen_write_native_ep | 1.007x | 0.983x | 1.911x |
| snapshot | 1.005x | 1.000x | 1.002x |
| san_parse_all | 0.929x | 0.903x | 1.775x |
| san_render_native | 0.992x | 0.966x | 2.142x |
| apply_children_full | 1.004x | 1.001x | 1.364x |
| make_unmake_full | 1.062x | 1.006x | 1.304x |
| startpos | 0.999x | 0.999x | 1.348x |
| chess960_materialize | 1.004x | 0.995x | 1.567x |
| chess960_perft_d3 | 1.002x | 0.988x | 1.498x |
| polyglot_key | 1.066x | 1.004x | 41.012x |
| polyglot_recompute | 1.020x | 0.996x | 0.974x |
| repetition_key | 1.098x | 0.912x | 22.095x |
| corpus_count | 1.146x | 1.086x | 1.310x |
| corpus_materialize | 0.999x | 0.996x | 1.138x |
| corpus_uci_sorted | 0.985x | 0.982x | 1.554x |
| replay_trusted_native | 1.014x | 1.000x | 0.916x |
| replay_checked_native | 1.003x | 1.003x | 1.133x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.15x).
