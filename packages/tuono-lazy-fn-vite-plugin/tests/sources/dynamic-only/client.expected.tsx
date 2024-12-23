import React, { Suspense, type JSX } from "react";
import { __tuono__internal__lazyLoadComponent as dynamic } from "tuono";
const DynamicComponent = dynamic(() => import("../components/DynamicComponent"));
const Loading = (): JSX.Element => <>Loading</>;
export default function IndexPage(): JSX.Element {
  return <Suspense fallback={<Loading />}>
			<DynamicComponent />
		</Suspense>;
}