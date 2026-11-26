| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.998x | 1.485x |
| materialize | 0.997x | 0.977x | 1.201x |
| perft_bulk_d4 | 1.013x | 0.997x | 1.324x |
| perft_full_d3 | 1.342x | 1.085x | 1.617x |
| fen_parse | 0.795x | 0.778x | 0.940x |
| fen_write_native_ep | 0.999x | 0.987x | 1.871x |
| snapshot | 1.000x | 1.000x | 1.251x |
| san_parse_all | 1.001x | 0.982x | 2.199x |
| san_render_native | 1.000x | 0.956x | 2.094x |
| apply_children_full | 1.026x | 0.980x | 1.327x |
| make_unmake_full | 0.981x | 0.914x | 1.189x |
| startpos | 1.000x | 1.000x | 1.199x |
| chess960_materialize | 0.984x | 0.983x | 1.561x |
| chess960_perft_d3 | 1.006x | 1.000x | 1.608x |
| polyglot_key | 0.995x | 0.989x | 20.390x |
| polyglot_recompute | 1.086x | 1.016x | 1.108x |
| repetition_key | 1.001x | 0.997x | 19.053x |
| corpus_count | 1.003x | 1.002x | 1.304x |
| corpus_materialize | 1.000x | 0.997x | 1.236x |
| corpus_uci_sorted | 0.978x | 0.970x | 1.637x |
| replay_trusted_native | 1.009x | 1.004x | 1.080x |
| replay_checked_native | 0.989x | 0.982x | 1.120x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).
