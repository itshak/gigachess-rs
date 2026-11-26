| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.997x | 1.188x |
| materialize | 1.009x | 0.992x | 1.290x |
| perft_bulk_d4 | 1.011x | 0.999x | 1.230x |
| perft_full_d3 | 1.011x | 0.987x | 1.343x |
| fen_parse | 1.076x | 1.060x | 1.245x |
| fen_write_native_ep | 1.006x | 1.000x | 1.714x |
| snapshot | 1.005x | 0.996x | 1.004x |
| san_parse_all | 0.979x | 0.960x | 1.996x |
| san_render_native | 1.005x | 0.988x | 2.413x |
| apply_children_full | 1.085x | 1.029x | 1.268x |
| make_unmake_full | 1.014x | 0.996x | 1.348x |
| startpos | 1.002x | 1.002x | 1.280x |
| chess960_materialize | 0.996x | 0.995x | 1.353x |
| chess960_perft_d3 | 0.999x | 0.996x | 1.378x |
| polyglot_key | 1.061x | 1.041x | 27.654x |
| polyglot_recompute | 1.018x | 1.013x | 1.021x |
| repetition_key | 1.064x | 1.014x | 20.072x |
| corpus_count | 0.998x | 0.996x | 1.283x |
| corpus_materialize | 1.020x | 1.017x | 1.179x |
| corpus_uci_sorted | 0.997x | 0.994x | 1.808x |
| replay_trusted_native | 1.078x | 1.046x | 0.757x |
| replay_checked_native | 0.997x | 0.988x | 1.030x |

Cold first legal move list: 0.016 ms -> 0.015 ms (1.07x).
