| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.56x | 1.88x | 1.65x | 2.53x | 1.93x | 30/30 |
| materialize | 1.01x | 2.26x | 1.69x | 1.76x | 2.66x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.51x | 1.21x | 2.34x | 1.85x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.61x | 1.54x | 1.98x | 1.68x | 30/30 |
| fen_parse | 0.94x | 1.39x | 1.28x | 1.10x | 1.38x | 1.17x | 30/30 |
| fen_write_native_ep | 1.01x | 4.92x | 10.58x | 2.04x | 27.85x | 2.49x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.02x | 1.91x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.19x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 0.81x | 1.70x | 1.09x | 3.28x | 1.09x | 1.35x | 28/30 |
| make_unmake_full | 0.85x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.12x | 88.24x | 161.89x | 189.91x | 169.87x | 5/5 |
| chess960_materialize | 1.01x | 1.77x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.57x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 39.91x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.06x | 23.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.17x | 1.95x | 1.07x | 2.37x | 2.18x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.10x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.34x | 3.08x | 2.27x | 2.98x | 1.57x | 10/10 |
| replay_trusted_native | 0.83x | 0.74x | 1.37x | 1.35x | 1.86x | 2.15x | 8/10 |
| replay_checked_native | 0.95x | 2.11x | 1.13x | 8.72x | 8.41x | 9.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
