| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 5.08x | 1.78x | 1.76x | 2.42x | 2.01x | 30/30 |
| materialize | 1.02x | 2.00x | 1.61x | 1.07x | 2.21x | 1.40x | 27/30 |
| perft_bulk_d4 | 1.12x | 4.12x | 1.51x | 1.31x | 1.91x | 1.87x | 30/30 |
| perft_full_d3 | 0.98x | 1.40x | 1.64x | 1.29x | 1.86x | 1.91x | 30/30 |
| fen_parse | 1.10x | 1.22x | 1.48x | 1.25x | 1.60x | 1.35x | 29/30 |
| fen_write_native_ep | 1.08x | 4.70x | 8.88x | 1.78x | 27.55x | 2.27x | 30/30 |
| snapshot | 1.04x | 1.74x | 1.11x | 2.68x | 1.14x | 0.99x | 27/30 |
| san_parse_all | 1.17x | 1.78x | N/A | 4.12x | N/A | N/A | 12/12 |
| san_render_native | 1.21x | 12.75x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.23x | 1.22x | 1.27x | 6.06x | 1.26x | 1.41x | 29/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.23x | 71.34x | 131.76x | 162.79x | 144.58x | 5/5 |
| chess960_materialize | 0.93x | 1.73x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.09x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 38.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 21.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 6.33x | 1.92x | 1.19x | 2.27x | 2.44x | 10/10 |
| corpus_materialize | 1.07x | 1.50x | 1.43x | 1.15x | 1.92x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.20x | 1.92x | 2.76x | 1.88x | 2.51x | 1.41x | 10/10 |
| replay_trusted_native | 1.19x | 0.65x | 1.77x | 1.82x | 1.91x | 1.88x | 8/10 |
| replay_checked_native | 1.02x | 2.82x | 1.30x | 10.59x | 9.03x | 11.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
