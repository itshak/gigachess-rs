| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 5.41x | 1.73x | 1.82x | 2.49x | 2.12x | 30/30 |
| materialize | 1.09x | 2.12x | 1.64x | 1.07x | 2.28x | 1.41x | 27/30 |
| perft_bulk_d4 | 1.15x | 4.08x | 1.52x | 1.26x | 1.90x | 1.82x | 30/30 |
| perft_full_d3 | 1.03x | 1.45x | 1.61x | 1.24x | 1.83x | 1.92x | 29/30 |
| fen_parse | 1.04x | 1.22x | 1.36x | 1.32x | 1.58x | 1.41x | 30/30 |
| fen_write_native_ep | 1.09x | 4.43x | 9.03x | 1.80x | 28.03x | 2.25x | 30/30 |
| snapshot | 1.02x | 1.68x | 1.13x | 2.42x | 1.11x | 1.00x | 27/30 |
| san_parse_all | 1.08x | 1.70x | N/A | 3.56x | N/A | N/A | 12/12 |
| san_render_native | 1.09x | 11.05x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.27x | 1.24x | 1.36x | 6.50x | 1.36x | 1.44x | 30/30 |
| make_unmake_full | 1.17x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.27x | 75.06x | 134.37x | 175.85x | 141.35x | 5/5 |
| chess960_materialize | 1.08x | 1.87x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.13x | 3.39x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 39.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.16x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 20.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.65x | 1.74x | 1.05x | 2.08x | 2.21x | 9/10 |
| corpus_materialize | 1.10x | 1.45x | 1.48x | 1.07x | 1.90x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.10x | 1.87x | 2.68x | 1.89x | 2.44x | 1.39x | 10/10 |
| replay_trusted_native | 1.27x | 0.77x | 2.03x | 1.93x | 2.08x | 2.20x | 9/10 |
| replay_checked_native | 0.98x | 2.44x | 1.13x | 9.03x | 7.48x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
