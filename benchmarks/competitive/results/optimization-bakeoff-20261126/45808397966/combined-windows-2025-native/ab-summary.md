| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.035x | 0.916x | 1.418x |
| materialize | 1.013x | 0.903x | 1.089x |
| perft_bulk_d4 | 1.019x | 0.942x | 1.330x |
| perft_full_d3 | 1.072x | 1.024x | 1.066x |
| fen_parse | 1.041x | 0.987x | 1.482x |
| fen_write_native_ep | 1.006x | 0.997x | 1.647x |
| snapshot | 1.254x | 1.242x | 1.200x |
| san_parse_all | 1.042x | 1.024x | 1.980x |
| san_render_native | 0.986x | 0.931x | 3.337x |
| apply_children_full | 1.040x | 1.033x | 1.117x |
| make_unmake_full | 0.992x | 0.963x | 1.247x |
| startpos | 1.197x | 1.197x | 1.197x |
| chess960_materialize | 1.105x | 1.046x | 1.282x |
| chess960_perft_d3 | 1.099x | 0.997x | 1.547x |
| polyglot_key | 0.996x | 0.986x | 20.329x |
| polyglot_recompute | 0.992x | 0.978x | 0.984x |
| repetition_key | 0.940x | 0.884x | 15.776x |
| corpus_count | 0.931x | 0.834x | 1.230x |
| corpus_materialize | 0.986x | 0.980x | 1.144x |
| corpus_uci_sorted | 0.956x | 0.920x | 1.450x |
| replay_trusted_native | 0.986x | 0.968x | 0.780x |
| replay_checked_native | 0.993x | 0.992x | 0.911x |

Cold first legal move list: 0.021 ms -> 0.013 ms (1.66x).
