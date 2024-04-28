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
		},
		{ duration: 500 }
	)
</script>

<div class="grow flex gap-4">
	<Code
		lang="c"
		lines="1-11|8|9|10"
		steps={[
			[
				'8',
				async () =>
					await stackPointer.to({ x: 180, textOpacity1: 1, textOpacity2: 0 }),
			],
			[
				'9',
				async () =>
					await stackPointer.to({
						textOpacity2: 1,
						targetX: 175,
						targetY: 120,
					}),
			],
			[
				'10',
				async () => {
					await stackPointer.to({ x: 0, textOpacity1: 0 })
					// stackPointer.reset()
				},
			],
		]}
		class="basis-[500px] mt-[50px] ml-[50px] grow"
	>
		{`
			struct vector {
			  int* data;
			  size_t size;
			  size_t capacity;
			};
	
			void foo() {
			  struct vector v = {NULL, 0U, 0U};
			  vector_push_back(&v, 33);
			  return; // oops!
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
			x="60"
			y="100"
			yOffset="1"
			textOpacity={$stackPointer.textOpacity1.toString()}
			text="data"
		/>
		<BoxedText
			width="60"
			height="30"
			x="120"
			y="100"
			yOffset="1"
			textOpacity={$stackPointer.textOpacity1.toString()}
			text="size"
		/>
		<BoxedText
			width="60"
			height="30"
			x="180"
			y="100"
			yOffset="1"
			textOpacity={$stackPointer.textOpacity1.toString()}
			text="cap"
		/>
		<BoxedText
			width="30"
			height="30"
			x="250"
			y="250"
			yOffset="1"
			strokeOpacity={$stackPointer.textOpacity2.toString()}
			textOpacity={$stackPointer.textOpacity2.toString()}
			text="33"
		/>
		<line
			x1="90"
			y1="130"
			x2={90 + $stackPointer.targetX}
			y2={130 + $stackPointer.targetY}
			opacity={$stackPointer.textOpacity1.toString()}
			stroke-linecap="round"
			stroke-linejoin="round"
		/>
		<g transform="translate({75 + $stackPointer.x}, 140)">
			<polyline
				points="-5 5 0 0 5 5"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		</g>
	</svg>
</div>
