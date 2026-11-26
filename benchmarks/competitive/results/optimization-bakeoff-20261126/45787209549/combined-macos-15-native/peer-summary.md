| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 4.41x | 1.48x | 1.62x | 2.02x | 1.72x | 30/30 |
| materialize | 0.82x | 1.78x | 1.45x | 1.11x | 2.23x | 1.32x | 26/30 |
| perft_bulk_d4 | 0.89x | 4.42x | 1.83x | 1.42x | 2.21x | 1.90x | 30/30 |
| perft_full_d3 | 0.83x | 1.54x | 1.63x | 1.41x | 1.98x | 1.97x | 29/30 |
| fen_parse | 0.83x | 1.12x | 1.27x | 1.26x | 1.53x | 1.49x | 28/30 |
| fen_write_native_ep | 0.85x | 4.04x | 8.19x | 1.63x | 26.84x | 2.09x | 30/30 |
| snapshot | 0.93x | 1.73x | 1.23x | 2.25x | 1.17x | 1.06x | 26/30 |
| san_parse_all | 0.89x | 1.94x | N/A | 4.53x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 11.82x | N/A | 2.79x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.04x | 1.07x | 5.26x | 1.01x | 1.19x | 24/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.23x | N/A | N/A | 5/6 |
| startpos | 1.11x | 1.32x | 113.55x | 191.06x | 202.25x | 184.09x | 5/5 |
| chess960_materialize | 1.20x | 1.69x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 3.51x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 46.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.86x | 6.59x | 2.44x | 1.30x | 2.57x | 2.77x | 10/10 |
| corpus_materialize | 0.85x | 1.40x | 1.24x | 0.94x | 1.82x | 1.18x | 8/10 |
| corpus_uci_sorted | 0.95x | 1.83x | 2.86x | 2.00x | 2.73x | 1.52x | 10/10 |
| replay_trusted_native | 1.02x | 0.51x | 1.27x | 1.25x | 1.47x | 1.55x | 8/10 |
| replay_checked_native | 0.87x | 2.23x | 1.04x | 7.27x | 6.87x | 7.37x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
