| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.89x | 1.68x | 1.42x | 2.14x | 1.72x | 30/30 |
| materialize | 1.07x | 2.22x | 1.60x | 1.67x | 2.48x | 1.78x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.32x | 1.38x | 1.04x | 2.09x | 1.64x | 29/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.68x | 1.60x | 2.06x | 1.75x | 30/30 |
| fen_parse | 0.93x | 1.18x | 1.47x | 1.08x | 1.49x | 1.16x | 29/30 |
| fen_write_native_ep | 1.04x | 4.59x | 10.14x | 1.92x | 28.31x | 2.17x | 30/30 |
| snapshot | 1.01x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 23/30 |
| san_parse_all | 1.15x | 2.10x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.25x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.91x | 1.18x | 3.57x | 1.20x | 1.48x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.25x | 1.26x | 110.44x | 183.65x | 213.44x | 191.59x | 5/5 |
| chess960_materialize | 1.01x | 1.63x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.40x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 35.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.89x | 16.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.15x | 1.57x | 0.86x | 1.86x | 1.73x | 8/10 |
| corpus_materialize | 1.01x | 1.36x | 1.25x | 1.04x | 1.66x | 1.13x | 9/10 |
| corpus_uci_sorted | 1.00x | 2.27x | 2.94x | 2.21x | 2.81x | 1.58x | 10/10 |
| replay_trusted_native | 1.02x | 0.81x | 1.52x | 1.48x | 2.08x | 2.66x | 9/10 |
| replay_checked_native | 0.99x | 2.04x | 1.16x | 8.73x | 8.43x | 10.10x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
