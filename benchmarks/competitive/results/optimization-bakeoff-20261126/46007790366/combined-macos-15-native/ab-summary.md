| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.958x | 0.748x | 1.328x |
| materialize | 0.986x | 0.744x | 1.031x |
| perft_bulk_d4 | 0.971x | 0.653x | 1.218x |
| perft_full_d3 | 1.267x | 0.960x | 1.576x |
| fen_parse | 0.982x | 0.752x | 1.234x |
| fen_write_native_ep | 0.979x | 0.812x | 1.577x |
| snapshot | 0.944x | 0.562x | 0.937x |
| san_parse_all | 1.117x | 0.941x | 1.799x |
| san_render_native | 1.007x | 0.852x | 2.888x |
| apply_children_full | 1.007x | 0.840x | 1.074x |
| make_unmake_full | 1.049x | 0.828x | 1.247x |
| startpos | 1.467x | 1.467x | 1.159x |
| chess960_materialize | 1.277x | 1.227x | 1.560x |
| chess960_perft_d3 | 1.177x | 1.134x | 1.523x |
| polyglot_key | 1.297x | 1.191x | 41.065x |
| polyglot_recompute | 1.190x | 0.940x | 1.105x |
| repetition_key | 1.122x | 1.062x | 22.205x |
| corpus_count | 1.166x | 1.104x | 1.220x |
| corpus_materialize | 1.292x | 1.239x | 1.369x |
| corpus_uci_sorted | 0.996x | 0.989x | 1.220x |
| replay_trusted_native | 1.030x | 1.016x | 0.614x |
| replay_checked_native | 1.176x | 1.110x | 1.353x |

Cold first legal move list: 0.003 ms -> 0.006 ms (0.55x).
