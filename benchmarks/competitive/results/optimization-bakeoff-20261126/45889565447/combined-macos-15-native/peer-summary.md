| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 5.16x | 1.71x | 1.74x | 2.42x | 2.07x | 30/30 |
| materialize | 1.04x | 2.21x | 1.60x | 1.17x | 2.29x | 1.49x | 29/30 |
| perft_bulk_d4 | 0.92x | 3.92x | 1.44x | 1.11x | 1.81x | 1.65x | 28/30 |
| perft_full_d3 | 1.08x | 1.68x | 1.96x | 1.51x | 2.16x | 2.29x | 30/30 |
| fen_parse | 0.94x | 1.15x | 1.22x | 1.32x | 1.69x | 1.56x | 30/30 |
| fen_write_native_ep | 1.07x | 4.19x | 9.34x | 1.87x | 28.61x | 2.28x | 30/30 |
| snapshot | 1.04x | 1.85x | 1.17x | 2.75x | 1.13x | 1.14x | 28/30 |
| san_parse_all | 0.95x | 1.68x | N/A | 3.52x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.98x | N/A | 2.83x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.14x | 1.17x | 5.56x | 1.10x | 1.24x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.18x | 1.24x | 83.84x | 153.04x | 187.01x | 172.74x | 5/5 |
| chess960_materialize | 1.16x | 1.84x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.20x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 45.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.91x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.03x | 19.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 5.46x | 1.92x | 1.09x | 1.94x | 2.22x | 9/10 |
| corpus_materialize | 1.06x | 1.49x | 1.42x | 1.11x | 1.92x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.85x | 2.64x | 1.92x | 2.48x | 1.36x | 10/10 |
| replay_trusted_native | 0.92x | 0.62x | 1.75x | 1.68x | 2.11x | 1.87x | 8/10 |
| replay_checked_native | 0.87x | 2.66x | 1.18x | 8.89x | 7.60x | 9.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
