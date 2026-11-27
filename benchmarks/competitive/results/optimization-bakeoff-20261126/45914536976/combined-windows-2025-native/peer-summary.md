| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.83x | 1.59x | 2.18x | 2.00x | 30/30 |
| materialize | 1.01x | 2.08x | 1.42x | 1.59x | 2.48x | 1.66x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.97x | 1.53x | 1.16x | 2.13x | 1.84x | 29/30 |
| perft_full_d3 | 1.00x | 2.16x | 1.60x | 1.43x | 2.06x | 1.67x | 30/30 |
| fen_parse | 0.87x | 1.06x | 1.28x | 1.09x | 1.53x | 1.27x | 30/30 |
| fen_write_native_ep | 1.00x | 3.92x | 7.86x | 1.69x | 30.06x | 2.16x | 30/30 |
| snapshot | 1.13x | 2.06x | 1.11x | 4.83x | 1.10x | 1.11x | 30/30 |
| san_parse_all | 0.96x | 1.66x | N/A | 4.97x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.73x | N/A | 4.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.08x | 1.35x | 10.50x | 1.33x | 1.66x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.00x | 78.65x | 149.53x | 213.33x | 181.02x | 4/5 |
| chess960_materialize | 1.05x | 1.59x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.54x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 27.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.64x | 1.90x | 0.94x | 2.11x | 2.03x | 8/10 |
| corpus_materialize | 1.03x | 1.44x | 1.24x | 1.09x | 1.77x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.84x | 2.18x | 1.90x | 2.12x | 1.58x | 10/10 |
| replay_trusted_native | 1.01x | 0.84x | 1.57x | 1.62x | 2.17x | 2.54x | 9/10 |
| replay_checked_native | 1.03x | 2.29x | 1.21x | 9.24x | 9.03x | 10.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
