<script lang="ts">
	import Code from '@lib/components/code.svelte'
	import { signal } from '@motion'
	import BoxedText from '../../boxedText.svelte'

	const stackPointer = signal(
		{ x: 0, textOpacity1: 0, dataX: -120, textOpacity2: 0 },
		{ duration: 500 }
	)
</script>

<div class="grow flex gap-4">
	<Code
		lang="c"
		lines="1-4|2|3|4"
		steps={[
			[
				'2',
				async () =>
					await stackPointer.to({ x: 120, textOpacity1: 1, textOpacity2: 0 }),
			],
			[
				'3',
				async () => {
					await stackPointer.to({ x: 240, textOpacity1: 1, textOpacity2: 1 })
					await stackPointer.to({
						x: 240,
						dataX: 0,
						textOpacity1: 1,
						textOpacity2: 1,
					})
				},
			],
			[
				'4',
				async () => {
					await stackPointer.to({ x: 0, textOpacity1: 0, textOpacity2: 0 })
					stackPointer.reset()
				},
			],
		]}
		class="basis-[300px] mt-[100px] ml-[200px] grow"
	>
		{`
        void foo() {
          int i = 2;
          int other = i;
          return;
        }
        `}
	</Code>
	<svg
		data-id="animation"
		class="basis-[500px] stroke-white stroke-2 fill-transparent"
	>
		<BoxedText
			width="30"
			x="10"
			y="150"
			yOffset="1"
			text="02"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<BoxedText
			width="30"
			x="40"
			y="150"
			yOffset="1"
			text="00"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<BoxedText
			width="30"
			x="70"
			y="150"
			yOffset="1"
			text="00"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<BoxedText
			width="30"
			x="100"
			y="150"
			yOffset="1"
			text="00"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<BoxedText
			width="30"
			x="130"
			xOffset={$stackPointer.dataX.toString()}
			y="150"
			yOffset="1"
			text="02"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<BoxedText
			width="30"
			x="160"
			xOffset={$stackPointer.dataX.toString()}
			y="150"
			yOffset="1"
			text="00"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<BoxedText
			width="30"
			x="190"
			xOffset={$stackPointer.dataX.toString()}
			y="150"
			yOffset="1"
			text="00"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<BoxedText
			width="30"
			x="220"
			xOffset={$stackPointer.dataX.toString()}
			y="150"
			yOffset="1"
			text="00"
			textOpacity={$stackPointer.textOpacity1.toString()}
		/>
		<rect width="30" height="30" x="250" y="150" />
		<g transform="translate({25 + $stackPointer.x}, 190)">
			<polyline
				points="-5 5 0 0 5 5"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		</g>
	</svg>
</div>
