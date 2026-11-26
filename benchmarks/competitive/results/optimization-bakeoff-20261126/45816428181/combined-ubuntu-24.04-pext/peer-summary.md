| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.11x | 1.71x | 1.50x | 2.30x | 1.76x | 30/30 |
| materialize | 0.97x | 2.16x | 1.58x | 1.64x | 2.55x | 1.80x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.41x | 1.36x | 1.06x | 2.10x | 1.67x | 29/30 |
| perft_full_d3 | 1.00x | 1.50x | 1.65x | 1.60x | 2.13x | 1.75x | 30/30 |
| fen_parse | 0.93x | 1.26x | 1.47x | 1.01x | 1.36x | 1.07x | 28/30 |
| fen_write_native_ep | 1.01x | 4.69x | 10.16x | 2.14x | 28.48x | 2.70x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 1.93x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.08x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 2.05x | 1.29x | 3.87x | 1.30x | 1.63x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.12x | 88.16x | 154.85x | 188.82x | 154.62x | 5/5 |
| chess960_materialize | 1.02x | 1.56x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.36x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 37.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 15.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.69x | 4.69x | 1.73x | 0.96x | 2.13x | 1.94x | 8/10 |
| corpus_materialize | 0.95x | 1.37x | 1.27x | 1.05x | 1.70x | 1.14x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.27x | 3.01x | 2.24x | 2.84x | 1.55x | 10/10 |
| replay_trusted_native | 1.08x | 0.90x | 1.61x | 1.56x | 2.20x | 2.53x | 9/10 |
| replay_checked_native | 0.99x | 2.05x | 1.12x | 8.73x | 8.51x | 9.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
