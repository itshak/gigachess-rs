| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.43x | 1.69x | 1.82x | 2.19x | 2.19x | 30/30 |
| materialize | 1.02x | 1.97x | 1.54x | 1.36x | 2.18x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.02x | 1.63x | 1.48x | 2.09x | 2.14x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.81x | 1.50x | 2.21x | 2.10x | 30/30 |
| fen_parse | 1.02x | 1.25x | 1.34x | 1.28x | 1.79x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 4.51x | 9.33x | 1.58x | 30.03x | 2.40x | 30/30 |
| snapshot | 0.98x | 1.05x | 1.09x | 1.45x | 1.10x | 0.96x | 24/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.38x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 3.99x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.70x | 127.89x | 169.74x | 135.74x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 26.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.37x | 1.48x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 4.85x | 1.84x | 1.44x | 2.13x | 2.37x | 10/10 |
| corpus_materialize | 1.00x | 1.63x | 1.54x | 1.22x | 1.90x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.92x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.66x | 2.04x | 2.26x | 2.32x | 8/10 |
| replay_checked_native | 1.01x | 2.38x | 1.15x | 7.66x | 6.92x | 8.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
