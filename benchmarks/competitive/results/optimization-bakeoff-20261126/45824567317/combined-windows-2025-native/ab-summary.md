| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.998x | 1.323x |
| materialize | 1.033x | 0.995x | 1.184x |
| perft_bulk_d4 | 1.068x | 0.976x | 1.196x |
| perft_full_d3 | 1.004x | 0.969x | 1.245x |
| fen_parse | 1.046x | 1.011x | 1.286x |
| fen_write_native_ep | 1.059x | 1.020x | 1.642x |
| snapshot | 1.010x | 0.982x | 0.996x |
| san_parse_all | 1.020x | 0.997x | 2.063x |
| san_render_native | 1.013x | 0.980x | 3.878x |
| apply_children_full | 1.070x | 1.027x | 1.400x |
| make_unmake_full | 1.061x | 0.971x | 1.353x |
| startpos | 1.003x | 1.003x | 1.250x |
| chess960_materialize | 1.008x | 0.998x | 1.281x |
| chess960_perft_d3 | 1.183x | 1.115x | 1.371x |
| polyglot_key | 0.935x | 0.916x | 27.449x |
| polyglot_recompute | 1.021x | 0.998x | 1.017x |
| repetition_key | 0.982x | 0.956x | 20.313x |
| corpus_count | 1.060x | 1.008x | 1.256x |
| corpus_materialize | 0.997x | 0.995x | 1.081x |
| corpus_uci_sorted | 1.037x | 0.997x | 1.399x |
| replay_trusted_native | 1.013x | 1.010x | 0.837x |
| replay_checked_native | 1.127x | 1.051x | 1.110x |

Cold first legal move list: 0.014 ms -> 0.017 ms (0.84x).
