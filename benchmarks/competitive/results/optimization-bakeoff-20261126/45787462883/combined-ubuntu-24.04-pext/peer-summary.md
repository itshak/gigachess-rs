| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.70x | 1.62x | 1.65x | 2.56x | 1.93x | 30/30 |
| materialize | 0.99x | 2.13x | 1.67x | 1.15x | 2.20x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.78x | 3.26x | 1.26x | 1.04x | 2.08x | 1.65x | 27/30 |
| perft_full_d3 | 0.92x | 1.16x | 1.19x | 0.91x | 1.64x | 1.47x | 25/30 |
| fen_parse | 0.96x | 1.28x | 1.46x | 1.07x | 2.01x | 1.06x | 29/30 |
| fen_write_native_ep | 1.02x | 4.08x | 10.14x | 1.86x | 27.74x | 2.13x | 30/30 |
| snapshot | 1.22x | 1.73x | 1.19x | 2.64x | 1.21x | 1.10x | 30/30 |
| san_parse_all | 0.93x | 2.15x | N/A | 3.52x | N/A | N/A | 12/12 |
| san_render_native | 0.85x | 8.94x | N/A | 1.80x | N/A | N/A | 12/12 |
| apply_children_full | 0.69x | 1.07x | 0.71x | 2.72x | 0.80x | 0.94x | 11/30 |
| make_unmake_full | 0.63x | N/A | N/A | 0.73x | N/A | N/A | 1/6 |
| startpos | 1.09x | 1.18x | 87.13x | 130.94x | 213.72x | 129.85x | 5/5 |
| chess960_materialize | 0.98x | 1.84x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.85x | 2.54x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.03x | 1.92x | 1.48x | 2.65x | 2.18x | 10/10 |
| corpus_materialize | 1.01x | 2.04x | 1.78x | 1.28x | 2.97x | 1.56x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.43x | 3.28x | 2.41x | 3.26x | 1.72x | 10/10 |
| replay_trusted_native | 0.77x | 0.58x | 0.97x | 1.11x | 2.02x | 2.33x | 7/10 |
| replay_checked_native | 0.84x | 2.12x | 0.79x | 4.53x | 5.63x | 6.05x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
