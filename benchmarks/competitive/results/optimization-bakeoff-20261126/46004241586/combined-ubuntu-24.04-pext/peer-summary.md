| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.70x | 1.96x | 1.71x | 2.63x | 1.99x | 30/30 |
| materialize | 1.09x | 2.29x | 1.69x | 1.80x | 2.67x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.01x | 1.59x | 1.25x | 2.45x | 1.95x | 29/30 |
| perft_full_d3 | 1.21x | 1.64x | 1.86x | 1.81x | 2.37x | 2.00x | 30/30 |
| fen_parse | 0.93x | 1.30x | 1.23x | 1.07x | 1.34x | 1.06x | 29/30 |
| fen_write_native_ep | 0.99x | 5.10x | 10.14x | 2.10x | 27.95x | 2.48x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.02x | 1.96x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.11x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.12x | 1.34x | 4.04x | 1.34x | 1.67x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.11x | 88.59x | 166.98x | 192.23x | 158.35x | 5/5 |
| chess960_materialize | 0.96x | 1.68x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.54x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.53x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 22.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.16x | 6.31x | 2.35x | 1.30x | 2.89x | 2.59x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.30x | 1.08x | 1.75x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.35x | 3.10x | 2.26x | 3.03x | 1.58x | 10/10 |
| replay_trusted_native | 1.00x | 0.91x | 1.60x | 1.65x | 2.23x | 2.60x | 9/10 |
| replay_checked_native | 1.00x | 2.20x | 1.15x | 9.24x | 8.86x | 10.36x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
