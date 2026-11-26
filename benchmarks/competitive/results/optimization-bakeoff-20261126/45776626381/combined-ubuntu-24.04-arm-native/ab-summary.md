| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.049x | 1.022x | 1.355x |
| materialize | 1.028x | 1.020x | 1.239x |
| perft_bulk_d4 | 1.058x | 1.034x | 1.348x |
| perft_full_d3 | 1.417x | 1.344x | 1.071x |
| fen_parse | 1.001x | 0.997x | 1.244x |
| fen_write_native_ep | 0.990x | 0.985x | 1.057x |
| snapshot | 0.983x | 0.980x | 0.893x |
| san_parse_all | 1.035x | 1.021x | 1.838x |
| san_render_native | 1.066x | 1.029x | 2.544x |
| apply_children_full | 1.233x | 1.174x | 1.045x |
| make_unmake_full | 1.247x | 1.197x | 1.449x |
| startpos | 1.000x | 1.000x | 1.184x |
| chess960_materialize | 1.074x | 1.021x | 1.317x |
| chess960_perft_d3 | 1.080x | 1.049x | 1.319x |
| polyglot_key | 1.003x | 0.992x | 25.790x |
| polyglot_recompute | 1.003x | 1.000x | 1.073x |
| repetition_key | 1.021x | 1.012x | 19.327x |
| corpus_count | 1.041x | 1.031x | 1.469x |
| corpus_materialize | 1.008x | 1.005x | 1.194x |
| corpus_uci_sorted | 1.503x | 1.497x | 1.460x |
| replay_trusted_native | 1.223x | 1.166x | 0.520x |
| replay_checked_native | 1.096x | 1.069x | 0.998x |

Cold first legal move list: 0.013 ms -> 0.009 ms (1.43x).
