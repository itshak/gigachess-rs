| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.980x | 0.872x | 1.417x |
| materialize | 0.967x | 0.744x | 1.028x |
| perft_bulk_d4 | 0.937x | 0.788x | 1.277x |
| perft_full_d3 | 0.997x | 0.839x | 1.373x |
| fen_parse | 0.985x | 0.783x | 1.170x |
| fen_write_native_ep | 0.884x | 0.803x | 1.177x |
| snapshot | 1.026x | 0.741x | 0.903x |
| san_parse_all | 1.006x | 0.812x | 1.952x |
| san_render_native | 0.966x | 0.921x | 2.473x |
| apply_children_full | 0.960x | 0.825x | 1.005x |
| make_unmake_full | 0.922x | 0.802x | 1.129x |
| startpos | 1.119x | 1.119x | 1.378x |
| chess960_materialize | 1.247x | 1.156x | 1.326x |
| chess960_perft_d3 | 1.106x | 1.014x | 1.652x |
| polyglot_key | 1.033x | 0.988x | 37.850x |
| polyglot_recompute | 1.031x | 1.019x | 1.049x |
| repetition_key | 1.100x | 0.995x | 21.344x |
| corpus_count | 1.248x | 1.047x | 1.285x |
| corpus_materialize | 0.988x | 0.643x | 1.040x |
| corpus_uci_sorted | 0.997x | 0.763x | 1.337x |
| replay_trusted_native | 0.943x | 0.863x | 0.606x |
| replay_checked_native | 1.043x | 0.964x | 0.896x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.08x).
