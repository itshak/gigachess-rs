| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.62x | 1.88x | 1.70x | 2.34x | 2.03x | 30/30 |
| materialize | 1.00x | 2.10x | 1.66x | 1.70x | 2.48x | 1.67x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.01x | 1.59x | 1.24x | 2.27x | 1.92x | 29/30 |
| perft_full_d3 | 1.23x | 1.70x | 2.09x | 1.83x | 2.39x | 2.04x | 30/30 |
| fen_parse | 1.01x | 1.41x | 1.50x | 1.25x | 1.57x | 1.30x | 30/30 |
| fen_write_native_ep | 1.01x | 4.92x | 10.91x | 2.14x | 29.63x | 2.65x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.79x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.44x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.12x | 1.35x | 4.13x | 1.34x | 1.70x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.12x | 92.24x | 164.97x | 200.86x | 169.06x | 5/5 |
| chess960_materialize | 0.98x | 1.62x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.58x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 25.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.20x | 5.76x | 2.27x | 1.14x | 2.59x | 2.45x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.32x | 1.08x | 1.71x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.23x | 2.97x | 2.17x | 2.87x | 1.55x | 10/10 |
| replay_trusted_native | 0.96x | 0.84x | 1.57x | 1.51x | 2.08x | 2.42x | 9/10 |
| replay_checked_native | 1.00x | 2.38x | 1.21x | 9.39x | 9.12x | 10.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
