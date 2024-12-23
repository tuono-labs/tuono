import React, { Suspense, type JSX } from "react";
import DynamicComponent from "../components/DynamicComponent";
const Loading = (): JSX.Element => <>Loading</>;
export default function IndexPage(): JSX.Element {
  return <Suspense fallback={<Loading />}>
			<DynamicComponent />
		</Suspense>;
}