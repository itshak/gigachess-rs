| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.54x | 1.66x | 1.90x | 2.33x | 2.14x | 30/30 |
| materialize | 1.01x | 1.90x | 1.58x | 1.50x | 2.15x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.26x | 1.83x | 1.57x | 2.46x | 2.38x | 30/30 |
| perft_full_d3 | 1.16x | 1.66x | 2.03x | 1.41x | 2.05x | 2.02x | 30/30 |
| fen_parse | 0.99x | 1.18x | 1.12x | 1.14x | 1.61x | 1.16x | 30/30 |
| fen_write_native_ep | 1.00x | 4.63x | 8.84x | 1.57x | 23.27x | 1.90x | 30/30 |
| snapshot | 1.00x | 1.05x | 1.08x | 1.19x | 1.08x | 0.99x | 24/30 |
| san_parse_all | 1.00x | 2.08x | N/A | 4.91x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.27x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.07x | 1.58x | 3.02x | 1.04x | 1.04x | 26/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.70x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 63.12x | 86.98x | 125.59x | 91.83x | 5/5 |
| chess960_materialize | 1.01x | 1.68x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.17x | 1.75x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 30.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 5.70x | 2.00x | 1.52x | 2.45x | 2.64x | 10/10 |
| corpus_materialize | 1.02x | 1.83x | 1.92x | 1.41x | 2.47x | 1.63x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 3.20x | 2.22x | 3.18x | 1.55x | 10/10 |
| replay_trusted_native | 1.02x | 0.67x | 1.76x | 2.03x | 2.68x | 3.31x | 8/10 |
| replay_checked_native | 1.03x | 2.66x | 1.28x | 9.51x | 8.27x | 11.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
