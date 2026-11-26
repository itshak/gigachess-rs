| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.06x | 1.70x | 1.48x | 2.33x | 1.74x | 30/30 |
| materialize | 0.97x | 2.14x | 1.60x | 1.68x | 2.53x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.83x | 3.38x | 1.35x | 1.06x | 2.10x | 1.64x | 29/30 |
| perft_full_d3 | 0.94x | 1.45x | 1.69x | 1.65x | 2.16x | 1.78x | 30/30 |
| fen_parse | 1.02x | 1.24x | 1.55x | 1.14x | 1.43x | 1.23x | 30/30 |
| fen_write_native_ep | 1.00x | 4.92x | 10.23x | 1.91x | 28.73x | 2.15x | 30/30 |
| snapshot | 1.82x | 2.00x | 1.00x | 4.66x | 1.00x | 1.34x | 24/30 |
| san_parse_all | 1.12x | 1.92x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.02x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.74x | 1.09x | 3.28x | 1.16x | 1.34x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.20x | 1.00x | 78.12x | 145.79x | 170.29x | 154.23x | 4/5 |
| chess960_materialize | 1.00x | 1.68x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.36x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 39.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.67x | 4.61x | 1.71x | 0.95x | 2.21x | 1.92x | 8/10 |
| corpus_materialize | 0.95x | 1.36x | 1.25x | 1.07x | 1.69x | 1.14x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.30x | 2.97x | 2.23x | 2.94x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.71x | 1.32x | 1.29x | 1.79x | 2.06x | 8/10 |
| replay_checked_native | 1.02x | 2.09x | 1.10x | 8.69x | 8.48x | 9.74x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
