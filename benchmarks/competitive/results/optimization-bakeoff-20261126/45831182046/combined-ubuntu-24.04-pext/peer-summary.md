| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.17x | 1.74x | 1.52x | 2.34x | 1.78x | 30/30 |
| materialize | 1.10x | 2.17x | 1.61x | 1.66x | 2.60x | 1.85x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.58x | 1.42x | 1.11x | 2.20x | 1.75x | 29/30 |
| perft_full_d3 | 1.02x | 1.54x | 1.66x | 1.63x | 2.15x | 1.79x | 30/30 |
| fen_parse | 1.09x | 1.36x | 1.61x | 1.08x | 1.47x | 1.16x | 30/30 |
| fen_write_native_ep | 1.04x | 4.62x | 10.11x | 2.16x | 28.03x | 2.65x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 1.00x | 1.90x | N/A | 4.46x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.19x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.13x | 1.34x | 4.07x | 1.35x | 1.69x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.32x | 152.55x | 191.33x | 157.21x | 5/5 |
| chess960_materialize | 1.13x | 1.72x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.43x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.36x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 18.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.61x | 1.70x | 0.95x | 2.11x | 1.91x | 8/10 |
| corpus_materialize | 1.00x | 1.41x | 1.29x | 1.06x | 1.72x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.30x | 3.01x | 2.26x | 2.90x | 1.59x | 10/10 |
| replay_trusted_native | 0.96x | 0.88x | 1.59x | 1.53x | 2.15x | 2.51x | 9/10 |
| replay_checked_native | 1.08x | 2.17x | 1.19x | 9.13x | 8.89x | 10.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
