| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.70x | 1.72x | 1.65x | 2.62x | 2.00x | 30/30 |
| materialize | 1.00x | 2.14x | 1.72x | 1.20x | 2.18x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.13x | 1.62x | 1.33x | 2.59x | 2.20x | 30/30 |
| perft_full_d3 | 1.34x | 2.02x | 2.09x | 1.62x | 3.06x | 2.53x | 30/30 |
| fen_parse | 0.80x | 1.13x | 1.22x | 0.94x | 1.59x | 1.13x | 25/30 |
| fen_write_native_ep | 1.00x | 4.08x | 10.20x | 1.87x | 28.82x | 2.10x | 30/30 |
| snapshot | 1.00x | 1.90x | 1.25x | 3.64x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.26x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.47x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.04x | 1.33x | 4.85x | 1.55x | 1.79x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 97.25x | 134.54x | 210.75x | 154.57x | 5/5 |
| chess960_materialize | 0.98x | 1.92x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.24x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.11x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.39x | 1.77x | 1.30x | 2.37x | 2.04x | 9/10 |
| corpus_materialize | 1.00x | 1.93x | 1.82x | 1.24x | 2.90x | 1.57x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.30x | 3.11x | 2.29x | 2.91x | 1.64x | 10/10 |
| replay_trusted_native | 1.01x | 1.08x | 1.89x | 2.11x | 3.62x | 4.44x | 9/10 |
| replay_checked_native | 0.99x | 2.87x | 1.12x | 6.16x | 7.01x | 8.38x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
