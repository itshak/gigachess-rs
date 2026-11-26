| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.27x | 1.82x | 1.55x | 2.18x | 1.89x | 30/30 |
| materialize | 1.00x | 1.99x | 1.58x | 1.60x | 2.28x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.75x | 1.52x | 1.16x | 2.13x | 2.02x | 29/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.61x | 1.50x | 1.93x | 1.71x | 30/30 |
| fen_parse | 1.01x | 1.29x | 1.51x | 1.09x | 1.52x | 1.31x | 30/30 |
| fen_write_native_ep | 1.00x | 4.88x | 10.05x | 2.11x | 28.71x | 2.73x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.70x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.56x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.99x | 1.27x | 3.87x | 1.26x | 1.60x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.10x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.12x | 91.19x | 144.15x | 199.22x | 175.16x | 5/5 |
| chess960_materialize | 0.97x | 1.51x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.55x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.78x | 24.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.76x | 1.85x | 0.94x | 2.12x | 2.02x | 8/10 |
| corpus_materialize | 0.99x | 1.38x | 1.27x | 1.05x | 1.67x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.30x | 2.98x | 2.27x | 2.86x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.47x | 1.50x | 2.10x | 2.49x | 9/10 |
| replay_checked_native | 0.99x | 2.15x | 1.07x | 8.64x | 8.41x | 10.31x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
