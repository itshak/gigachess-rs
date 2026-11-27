| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.962x | 0.682x | 1.542x |
| materialize | 1.033x | 0.752x | 1.124x |
| perft_bulk_d4 | 1.053x | 0.640x | 1.326x |
| perft_full_d3 | 1.022x | 0.913x | 1.311x |
| fen_parse | 1.122x | 0.968x | 1.249x |
| fen_write_native_ep | 1.065x | 0.880x | 1.702x |
| snapshot | 0.961x | 0.661x | 0.880x |
| san_parse_all | 1.031x | 0.886x | 1.950x |
| san_render_native | 0.976x | 0.859x | 3.009x |
| apply_children_full | 1.057x | 0.955x | 1.079x |
| make_unmake_full | 1.086x | 0.950x | 1.454x |
| startpos | 1.189x | 1.189x | 0.929x |
| chess960_materialize | 1.214x | 1.123x | 1.624x |
| chess960_perft_d3 | 1.323x | 1.286x | 1.842x |
| polyglot_key | 1.004x | 0.959x | 42.626x |
| polyglot_recompute | 1.002x | 0.912x | 0.922x |
| repetition_key | 1.100x | 0.900x | 22.224x |
| corpus_count | 0.747x | 0.679x | 0.759x |
| corpus_materialize | 1.093x | 0.983x | 1.304x |
| corpus_uci_sorted | 1.056x | 0.961x | 1.484x |
| replay_trusted_native | 1.099x | 0.984x | 0.672x |
| replay_checked_native | 0.892x | 0.747x | 1.082x |

Cold first legal move list: 0.008 ms -> 0.011 ms (0.73x).
