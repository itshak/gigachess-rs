| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 4.60x | 1.68x | 1.61x | 2.53x | 1.96x | 30/30 |
| materialize | 1.06x | 2.16x | 1.69x | 1.13x | 2.16x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.16x | 1.64x | 1.34x | 2.68x | 2.22x | 30/30 |
| perft_full_d3 | 1.03x | 1.29x | 1.34x | 1.00x | 1.85x | 1.58x | 26/30 |
| fen_parse | 1.07x | 1.34x | 1.52x | 1.12x | 2.16x | 1.29x | 30/30 |
| fen_write_native_ep | 1.00x | 4.05x | 9.81x | 1.79x | 26.98x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.73x | 1.22x | 2.79x | 1.25x | 1.11x | 30/30 |
| san_parse_all | 1.00x | 2.18x | N/A | 3.72x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.50x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.76x | 1.17x | 4.20x | 1.34x | 1.59x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 86.09x | 129.39x | 213.53x | 144.80x | 5/5 |
| chess960_materialize | 1.08x | 1.85x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.95x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.71x | 1.87x | 1.37x | 2.49x | 2.15x | 10/10 |
| corpus_materialize | 1.02x | 1.96x | 1.79x | 1.25x | 2.96x | 1.58x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.28x | 3.08x | 2.53x | 2.96x | 1.62x | 10/10 |
| replay_trusted_native | 1.02x | 0.82x | 1.42x | 1.58x | 2.91x | 3.56x | 9/10 |
| replay_checked_native | 1.00x | 2.52x | 0.98x | 5.43x | 6.52x | 7.71x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
