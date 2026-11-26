| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.12x | 1.73x | 1.50x | 2.30x | 1.76x | 30/30 |
| materialize | 1.00x | 2.07x | 1.67x | 1.71x | 2.65x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.97x | 1.57x | 1.24x | 2.44x | 1.92x | 30/30 |
| perft_full_d3 | 1.02x | 0.87x | 1.00x | 0.97x | 1.33x | 1.04x | 16/30 |
| fen_parse | 1.03x | 1.41x | 1.25x | 1.10x | 1.38x | 1.08x | 29/30 |
| fen_write_native_ep | 0.99x | 3.17x | 6.27x | 1.25x | 18.17x | 1.55x | 30/30 |
| snapshot | 1.28x | 1.41x | 1.15x | 3.21x | 1.15x | 1.15x | 30/30 |
| san_parse_all | 0.94x | 1.72x | N/A | 4.04x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 9.07x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.59x | 1.00x | 2.94x | 0.98x | 1.24x | 24/30 |
| make_unmake_full | 1.04x | N/A | N/A | 0.98x | N/A | N/A | 3/6 |
| startpos | 0.91x | 1.07x | 71.59x | 131.54x | 157.04x | 128.82x | 5/5 |
| chess960_materialize | 0.94x | 1.55x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.26x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 40.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.69x | 2.46x | 1.38x | 3.08x | 2.82x | 10/10 |
| corpus_materialize | 1.00x | 1.44x | 1.34x | 1.13x | 1.78x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.43x | 1.88x | 1.38x | 1.85x | 0.96x | 8/10 |
| replay_trusted_native | 1.00x | 0.60x | 1.12x | 1.11x | 1.53x | 1.74x | 7/10 |
| replay_checked_native | 0.98x | 1.79x | 0.95x | 7.72x | 7.28x | 8.86x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
