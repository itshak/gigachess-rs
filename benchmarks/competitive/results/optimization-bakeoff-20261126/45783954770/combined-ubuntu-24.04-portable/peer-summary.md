| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.69x | 1.53x | 1.32x | 1.77x | 1.28x | 29/30 |
| materialize | 0.93x | 2.06x | 1.64x | 1.75x | 2.47x | 1.83x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.95x | 1.56x | 1.17x | 1.95x | 1.74x | 30/30 |
| perft_full_d3 | 1.00x | 1.53x | 1.79x | 1.57x | 2.13x | 1.79x | 30/30 |
| fen_parse | 1.09x | 1.49x | 1.50x | 1.17x | 1.55x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 3.02x | 9.82x | 1.83x | 27.50x | 2.56x | 30/30 |
| snapshot | 1.25x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.07x | 1.91x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.22x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.70x | 1.15x | 3.26x | 1.08x | 1.22x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.35x | 57.00x | 108.29x | 129.26x | 100.79x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.87x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 20.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.65x | 2.32x | 1.27x | 2.38x | 3.18x | 10/10 |
| corpus_materialize | 1.00x | 1.54x | 1.45x | 1.15x | 1.88x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 2.94x | 2.21x | 2.94x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.68x | 1.31x | 1.33x | 1.79x | 1.64x | 8/10 |
| replay_checked_native | 1.00x | 1.92x | 1.09x | 8.48x | 8.14x | 9.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
