import type { NextPage } from "next";
import { memo, useState } from "react";

const Home: NextPage = memo(() => {
	const [count, setCount] = useState(0);

	const IncrementCount = async () => {
		try {
			const resp = await fetch("http://127.0.0.1:8000/increment", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({ count }),
			});
			const json = await resp.json();
			setCount(json.count);
		} catch (err) {
			console.log(err);
		}
	};

	return (
		<>
			<div className="flex min-h-screen flex-col items-center justify-center py-2">
				<button
					type="button"
					className="bg-purple-400 w-[200px] h-[45px] text-white rounded cursor-pointer hover:bg-purple-300 hover:duration-150"
					onClick={IncrementCount}
				>
					Increment!
				</button>
				{count}
			</div>
		</>
	);
});

export default Home;
