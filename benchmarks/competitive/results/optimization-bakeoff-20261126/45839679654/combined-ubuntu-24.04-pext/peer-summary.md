| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.42x | 1.85x | 1.61x | 2.48x | 1.89x | 30/30 |
| materialize | 0.91x | 1.96x | 1.52x | 1.65x | 2.45x | 1.81x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.71x | 1.47x | 1.16x | 2.27x | 1.80x | 29/30 |
| perft_full_d3 | 0.89x | 1.37x | 1.34x | 1.29x | 1.68x | 1.41x | 30/30 |
| fen_parse | 1.02x | 1.20x | 1.28x | 1.13x | 1.38x | 1.09x | 29/30 |
| fen_write_native_ep | 0.96x | 4.78x | 9.66x | 1.80x | 28.40x | 2.15x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 1.94x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 8.59x | N/A | 1.96x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.03x | 1.27x | 3.85x | 1.27x | 1.61x | 30/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.04x | N/A | N/A | 3/6 |
| startpos | 0.74x | 0.83x | 66.00x | 121.90x | 144.12x | 119.95x | 4/5 |
| chess960_materialize | 0.91x | 1.61x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.34x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.16x | 22.83x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 5.40x | 2.01x | 1.13x | 2.47x | 2.26x | 10/10 |
| corpus_materialize | 1.00x | 1.41x | 1.31x | 1.08x | 1.72x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 3.01x | 2.20x | 2.98x | 1.55x | 10/10 |
| replay_trusted_native | 0.94x | 0.84x | 1.48x | 1.52x | 2.08x | 2.39x | 9/10 |
| replay_checked_native | 0.96x | 1.93x | 1.03x | 8.13x | 7.82x | 9.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
