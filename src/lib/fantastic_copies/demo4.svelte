<script lang="ts">
	import Code from '@lib/components/code.svelte'
	import { signal } from '@motion'
	import BoxedText from '../../boxedText.svelte'

	const stackPointer = signal(
		{
			x: 0,
			textOpacity1: 0,
			dataX: -120,
			textOpacity2: 0,
			targetX: 0,
			targetY: 0,
			textOpacity3: 0,
			targetX3: 0,
			targetY3: 0,
			textOpacity4: 0,
			targetX4: 0,
			targetY4: 0,
		},
		{ duration: 500 }
	)
</script>

<div class="grow flex gap-4">
	<Code
		lang="cpp"
		lines="1-7|2|3|4|5"
		steps={[
			['2', async () => await stackPointer.to({ x: 180, textOpacity1: 1 })],
			[
				'3',
				async () =>
					await stackPointer.to({
						textOpacity2: 1,
						targetX: -50,
						targetY: 50,
					}),
			],
			[
				'4',
				async () =>
					await stackPointer.to({
						textOpacity3: 1,
						targetX3: -45,
						targetY3: 40,
					}),
			],
			[
				'5',
				async () => {
					await stackPointer.to({
						textOpacity4: 1,
						targetX: 0,
						targetY: 115,
					})
					await stackPointer.to({
						textOpacity2: 0.2,
						textOpacity3: 0.2,
					})
				},
			],
		]}
		class="basis-[500px] mt-[50px] ml-[50px] grow"
	>
		{`
			void foo() {
			  vector<vector<int>> v{};
			  v.emplace_back();
			  v.back().push_back(33);
			  v.emplace_back();
			  return;
			}
        `}
	</Code>
	<svg
		data-id="animation"
		class="basis-[500px] stroke-white stroke-2 fill-transparent"
	>
		<BoxedText
			width="60"
			height="30"
			x="110"
			y="100"
			yOffset="1"
			textOpacity={$stackPointer.textOpacity1.toString()}
			text="data"
		/>
		<BoxedText
			width="60"
			height="30"
			x="170"
			y="100"
			yOffset="1"
			textOpacity={$stackPointer.textOpacity1.toString()}
			text="size"
		/>
		<BoxedText
			width="60"
			height="30"
			x="230"
			y="100"
			yOffset="1"
			textOpacity={$stackPointer.textOpacity1.toString()}
			text="cap"
		/>
		<line
			x1="140"
			y1="130"
			x2={140 + $stackPointer.targetX}
			y2={130 + $stackPointer.targetY}
			opacity={$stackPointer.textOpacity1.toString()}
			stroke-linecap="round"
			stroke-linejoin="round"
		/>
		<BoxedText
			width="60"
			height="30"
			x="60"
			y="180"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity2.toString()}
			textOpacity={$stackPointer.textOpacity2.toString()}
			text="data"
		/>
		<BoxedText
			width="60"
			height="30"
			x="120"
			y="180"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity2.toString()}
			textOpacity={$stackPointer.textOpacity2.toString()}
			text="size"
		/>
		<BoxedText
			width="60"
			height="30"
			x="180"
			y="180"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity2.toString()}
			textOpacity={$stackPointer.textOpacity2.toString()}
			text="cap"
		/>
		<line
			x1="90"
			y1="210"
			x2={90 + $stackPointer.targetX3}
			y2={210 + $stackPointer.targetY3}
			opacity={$stackPointer.textOpacity2.toString()}
			stroke-linecap="round"
			stroke-linejoin="round"
		/>
		<BoxedText
			width="30"
			height="30"
			x="30"
			y="250"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity3.toString()}
			textOpacity={$stackPointer.textOpacity3.toString()}
			text="33"
		/>

		<!-- New vec<int> -->
		<BoxedText
			width="60"
			height="30"
			x="110"
			y="245"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity4.toString()}
			textOpacity={$stackPointer.textOpacity4.toString()}
			text="data"
		/>
		<BoxedText
			width="60"
			height="30"
			x="170"
			y="245"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity4.toString()}
			textOpacity={$stackPointer.textOpacity4.toString()}
			text="size"
		/>
		<BoxedText
			width="60"
			height="30"
			x="230"
			y="245"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity4.toString()}
			textOpacity={$stackPointer.textOpacity4.toString()}
			text="cap"
		/>
		<BoxedText
			width="60"
			height="30"
			x="290"
			y="245"
			yOffset="1"
			strokeOpacity={(0.2 * $stackPointer.textOpacity4).toString()}
			textOpacity={(0.2 * $stackPointer.textOpacity4).toString()}
			text="..."
		/>
		<line
			x1="140"
			y1="275"
			x2="145"
			y2="320"
			opacity={$stackPointer.textOpacity4.toString()}
			stroke-linecap="round"
			stroke-linejoin="round"
		/>
		<BoxedText
			width="30"
			height="30"
			x="130"
			y="320"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity4.toString()}
			textOpacity={$stackPointer.textOpacity4.toString()}
			text="33"
		/>
		<g transform="translate({125 + $stackPointer.x}, 140)">
			<polyline
				points="-5 5 0 0 5 5"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		</g>
	</svg>
</div>
