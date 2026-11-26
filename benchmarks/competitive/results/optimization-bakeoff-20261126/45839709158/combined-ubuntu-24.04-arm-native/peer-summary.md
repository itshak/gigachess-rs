| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.68x | 1.80x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.57x | 1.42x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 0.92x | 0.99x | 0.83x | 1.22x | 1.16x | 16/30 |
| fen_parse | 1.00x | 1.22x | 1.31x | 1.27x | 1.72x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.56x | 9.48x | 1.57x | 30.17x | 2.47x | 30/30 |
| snapshot | 1.00x | 1.13x | 0.99x | 1.49x | 1.01x | 0.93x | 17/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.76x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.03x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.22x | 130.65x | 176.50x | 142.85x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.99x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.97x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.14x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.41x | 1.72x | 1.34x | 2.02x | 2.21x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.52x | 1.21x | 1.91x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.61x | 1.62x | 1.95x | 2.20x | 2.23x | 8/10 |
| replay_checked_native | 1.00x | 2.14x | 1.04x | 6.97x | 6.25x | 8.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
