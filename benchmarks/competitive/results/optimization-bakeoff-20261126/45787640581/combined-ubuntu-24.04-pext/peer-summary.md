| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.11x | 1.72x | 1.48x | 2.32x | 1.74x | 30/30 |
| materialize | 0.94x | 1.99x | 1.47x | 1.64x | 2.37x | 1.77x | 30/30 |
| perft_bulk_d4 | 0.81x | 3.29x | 1.30x | 1.02x | 2.05x | 1.59x | 27/30 |
| perft_full_d3 | 0.94x | 1.46x | 1.71x | 1.62x | 2.12x | 1.74x | 30/30 |
| fen_parse | 0.99x | 1.42x | 1.32x | 1.14x | 1.42x | 1.12x | 29/30 |
| fen_write_native_ep | 0.97x | 4.42x | 10.00x | 1.85x | 27.83x | 2.45x | 30/30 |
| snapshot | 0.91x | 1.58x | 1.25x | 2.82x | 1.25x | 1.15x | 30/30 |
| san_parse_all | 1.04x | 1.77x | N/A | 3.98x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.12x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.75x | 1.15x | 3.26x | 1.09x | 1.38x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 80.65x | 146.83x | 173.97x | 148.78x | 5/5 |
| chess960_materialize | 0.92x | 1.59x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.37x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 37.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.05x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.67x | 4.58x | 1.69x | 0.94x | 2.10x | 1.93x | 8/10 |
| corpus_materialize | 0.95x | 1.38x | 1.28x | 1.08x | 1.69x | 1.14x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.25x | 2.91x | 2.19x | 2.85x | 1.55x | 10/10 |
| replay_trusted_native | 1.01x | 0.71x | 1.32x | 1.30x | 1.80x | 2.08x | 8/10 |
| replay_checked_native | 1.02x | 2.01x | 1.13x | 8.78x | 8.37x | 9.74x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
