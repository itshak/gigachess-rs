| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.48x | 1.93x | 1.64x | 2.30x | 1.99x | 30/30 |
| materialize | 1.00x | 2.02x | 1.58x | 1.65x | 2.37x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.03x | 1.63x | 1.24x | 2.27x | 1.96x | 29/30 |
| perft_full_d3 | 1.26x | 1.73x | 2.09x | 1.85x | 2.47x | 2.12x | 30/30 |
| fen_parse | 0.89x | 1.16x | 1.42x | 0.99x | 1.40x | 1.12x | 26/30 |
| fen_write_native_ep | 0.99x | 5.34x | 10.59x | 2.14x | 28.41x | 2.65x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.34x | 30/30 |
| san_parse_all | 1.01x | 1.84x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.28x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.10x | 1.37x | 4.12x | 1.32x | 1.71x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.80x | 1.00x | 81.67x | 133.20x | 177.32x | 143.71x | 5/5 |
| chess960_materialize | 1.03x | 1.63x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.56x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.94x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.24x | 31.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.40x | 6.55x | 2.54x | 1.30x | 2.91x | 2.77x | 10/10 |
| corpus_materialize | 1.01x | 1.39x | 1.30x | 1.05x | 1.67x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.37x | 2.98x | 2.22x | 2.90x | 1.57x | 10/10 |
| replay_trusted_native | 1.06x | 0.91x | 1.65x | 1.58x | 2.21x | 2.55x | 9/10 |
| replay_checked_native | 1.01x | 2.35x | 1.26x | 9.34x | 9.09x | 10.67x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
