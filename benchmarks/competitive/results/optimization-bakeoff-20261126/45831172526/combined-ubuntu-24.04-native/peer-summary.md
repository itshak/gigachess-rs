| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.68x | 1.43x | 2.19x | 1.73x | 30/30 |
| materialize | 1.01x | 2.10x | 1.60x | 1.61x | 2.48x | 1.80x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.34x | 1.37x | 1.04x | 2.04x | 1.64x | 29/30 |
| perft_full_d3 | 1.02x | 1.45x | 1.69x | 1.62x | 2.11x | 1.82x | 30/30 |
| fen_parse | 1.07x | 1.49x | 1.58x | 1.17x | 1.51x | 1.14x | 30/30 |
| fen_write_native_ep | 0.97x | 4.42x | 9.75x | 2.01x | 26.84x | 2.47x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.97x | 1.91x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.14x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.10x | 1.34x | 4.02x | 1.33x | 1.68x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.09x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.12x | 88.97x | 160.57x | 198.49x | 157.06x | 5/5 |
| chess960_materialize | 1.05x | 1.65x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.35x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 37.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.86x | 19.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.29x | 1.63x | 0.89x | 1.96x | 1.80x | 8/10 |
| corpus_materialize | 1.00x | 1.32x | 1.26x | 1.03x | 1.64x | 1.12x | 9/10 |
| corpus_uci_sorted | 1.02x | 2.27x | 2.97x | 2.21x | 2.83x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.90x | 1.62x | 1.62x | 2.22x | 2.62x | 9/10 |
| replay_checked_native | 0.99x | 1.99x | 1.10x | 8.34x | 7.95x | 9.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
