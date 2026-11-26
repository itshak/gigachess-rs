| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.86x | 1.68x | 1.43x | 1.91x | 1.40x | 30/30 |
| materialize | 1.09x | 2.26x | 1.92x | 1.85x | 2.92x | 2.08x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.02x | 1.60x | 1.19x | 1.98x | 1.81x | 29/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.58x | 1.42x | 1.89x | 1.60x | 30/30 |
| fen_parse | 1.00x | 1.52x | 1.46x | 1.19x | 1.51x | 1.19x | 30/30 |
| fen_write_native_ep | 0.98x | 2.94x | 9.73x | 1.75x | 26.85x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.76x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.00x | 1.89x | N/A | 4.11x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.30x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.12x | 1.40x | 4.00x | 1.30x | 1.51x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 56.99x | 106.78x | 129.42x | 105.54x | 5/5 |
| chess960_materialize | 1.00x | 1.85x | 1.78x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.13x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 42.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 19.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.14x | 2.09x | 1.13x | 2.09x | 2.89x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.41x | 1.15x | 1.86x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.99x | 2.93x | 2.24x | 2.91x | 1.55x | 10/10 |
| replay_trusted_native | 0.95x | 0.87x | 1.56x | 1.54x | 2.13x | 1.97x | 9/10 |
| replay_checked_native | 1.04x | 2.01x | 1.09x | 8.65x | 8.36x | 9.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
