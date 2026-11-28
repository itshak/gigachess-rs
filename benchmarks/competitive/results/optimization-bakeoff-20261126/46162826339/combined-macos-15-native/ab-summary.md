| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.938x | 0.785x | 1.628x |
| materialize | 0.948x | 0.817x | 1.174x |
| perft_bulk_d4 | 0.937x | 0.845x | 1.176x |
| perft_full_d3 | 1.146x | 1.012x | 1.410x |
| fen_parse | 1.042x | 0.874x | 1.261x |
| fen_write_native_ep | 1.090x | 0.838x | 1.755x |
| snapshot | 1.107x | 0.812x | 0.996x |
| san_parse_all | 0.997x | 0.806x | 1.699x |
| san_render_native | 1.009x | 0.939x | 3.032x |
| apply_children_full | 0.978x | 0.898x | 1.148x |
| make_unmake_full | 0.964x | 0.826x | 1.275x |
| startpos | 0.978x | 0.978x | 0.866x |
| chess960_materialize | 0.834x | 0.765x | 1.383x |
| chess960_perft_d3 | 0.960x | 0.915x | 1.566x |
| polyglot_key | 0.930x | 0.913x | 37.713x |
| polyglot_recompute | 1.447x | 1.358x | 1.465x |
| repetition_key | 1.157x | 1.118x | 21.069x |
| corpus_count | 1.441x | 1.079x | 1.242x |
| corpus_materialize | 1.169x | 1.039x | 1.264x |
| corpus_uci_sorted | 1.073x | 1.045x | 1.504x |
| replay_trusted_native | 0.908x | 0.864x | 0.704x |
| replay_checked_native | 0.891x | 0.867x | 1.220x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.76x).
