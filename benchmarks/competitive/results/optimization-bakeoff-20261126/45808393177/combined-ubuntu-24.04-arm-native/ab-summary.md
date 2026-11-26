| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 0.940x | 1.380x |
| materialize | 1.011x | 0.975x | 1.242x |
| perft_bulk_d4 | 0.919x | 0.897x | 1.238x |
| perft_full_d3 | 1.014x | 0.981x | 1.071x |
| fen_parse | 0.993x | 0.988x | 1.233x |
| fen_write_native_ep | 0.989x | 0.974x | 1.556x |
| snapshot | 0.916x | 0.913x | 0.946x |
| san_parse_all | 0.996x | 0.988x | 1.828x |
| san_render_native | 0.992x | 0.982x | 2.545x |
| apply_children_full | 1.159x | 1.124x | 1.298x |
| make_unmake_full | 1.109x | 1.086x | 1.593x |
| startpos | 1.073x | 1.073x | 1.308x |
| chess960_materialize | 1.026x | 0.958x | 1.361x |
| chess960_perft_d3 | 1.028x | 0.969x | 1.356x |
| polyglot_key | 1.013x | 1.008x | 25.689x |
| polyglot_recompute | 1.002x | 0.999x | 1.074x |
| repetition_key | 0.950x | 0.933x | 18.271x |
| corpus_count | 0.825x | 0.797x | 1.232x |
| corpus_materialize | 0.975x | 0.967x | 1.174x |
| corpus_uci_sorted | 0.997x | 0.994x | 1.469x |
| replay_trusted_native | 1.222x | 1.181x | 0.643x |
| replay_checked_native | 0.993x | 0.978x | 0.973x |

Cold first legal move list: 0.010 ms -> 0.005 ms (2.02x).
